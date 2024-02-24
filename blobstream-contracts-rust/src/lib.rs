#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub mod binary_merkle_tree;
pub mod signature;
// standard library
// use std::collections::HashMap;
// static allocator
// use std::alloc::{alloc, Layout};
use alloc::vec::Vec;
use alloy_primitives::fixed_bytes;
use std::str::FromStr;
use std::{collections::HashMap, mem::MaybeUninit};
// use alloy_primitives::utils::try_collect_vec;
// alloy
use alloy_primitives::{
    /*fixed_bytes, hex,*/ bytes, hex::FromHex, keccak256, FixedBytes, B256, U256,
};
use alloy_sol_macro::sol;
use alloy_sol_types::{SolType, SolValue};
// use alloy_sol_types::sol_data::FixedBytes;
// hash & curves
// use sha3::{Digest, Keccak256};
use signature::is_sig_nil;

// use alloy_sol_types::SolValue;

sol! {
    struct Validator {
        address addr;
        uint256 power;
    }
    struct Signature {
        uint8 v;
        bytes32 r;
        bytes32 s;
    }
    struct DataRootTuple{
        uint256 height;
        bytes32 dataRoot;
    }
    struct BinaryMerkleProof{
        bytes32[] sideNodes;
        uint256 key;
        uint256 numLeaves;
    }
    struct PackerDomainSeperatorValidator{
        bytes32 VALIDATOR_SET_HASH_DOMAIN_SEPARATOR;
        uint256 nonce;
        uint256 powerThreshold;
        bytes32 validatorSetHash;
    }
    struct PackerDomainSeperatorDataRootTuple{
        bytes32 DATA_ROOT_TUPLE_ROOT_DOMAIN_SEPERATOR;
        uint256 nonce;
        bytes32 dataRootTupleRoot;
    }
    struct PackerEthSignedMessage{
        string EIP191_SIGNED_MESSAGE;
        string len;
        bytes hash;
    }
    struct LeafDigestPacker{
        bytes1 leaf_prefix;
        bytes data;
    }
    struct NodeDigestPacker{
        bytes1 node_prefix;
        bytes32 left;
        bytes32 right;
    }
}

type SolArrayOf<T> = sol! { T[] };

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Allocates size bytes and leaks the pointer where they start.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "allocate_ptr")]
#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    // Allocate the amount of bytes needed.
    let vec: Vec<MaybeUninit<u8>> = Vec::with_capacity(size);

    // into_raw leaks the memory to the caller.
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

/// Deallocates size bytes at the pointer.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "deallocate_ptr")]
#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);
}

pub fn compute_validator_set_hash(validator: &Vec<Validator>) -> FixedBytes<32> {
    let tokenized_validator = SolArrayOf::<Validator>::abi_encode(validator);
    keccak256(tokenized_validator)
}

pub fn domain_seperator_validator_set_hash(
    nonce: U256,
    power_threshold: U256,
    validator_set_hash: FixedBytes<32>,
) -> FixedBytes<32> {
    // let validator_set_hash_domain_seperator = U256::from_str("checkpoint").unwrap().into();
    let validator_set_hash_domain_seperator =
        B256::from_hex("636865636b706f696e7400000000000000000000000000000000000000000000").unwrap();
    let abi_packed = PackerDomainSeperatorValidator {
        VALIDATOR_SET_HASH_DOMAIN_SEPARATOR: validator_set_hash_domain_seperator,
        nonce: nonce,
        powerThreshold: power_threshold,
        validatorSetHash: validator_set_hash,
    }
    .abi_encode();
    keccak256(abi_packed)
}

pub fn domain_seperator_data_root_tuple_root(
    nonce: U256,
    data_root_tuple_root: FixedBytes<32>,
) -> FixedBytes<32> {
    let data_root_tuple_root_domain_seperator =
        B256::from_hex("7472616e73616374696f6e426174636800000000000000000000000000000000").unwrap();
    let abi_packed = PackerDomainSeperatorDataRootTuple {
        DATA_ROOT_TUPLE_ROOT_DOMAIN_SEPERATOR: data_root_tuple_root_domain_seperator,
        nonce: nonce,
        dataRootTupleRoot: data_root_tuple_root,
    }
    .abi_encode();

    keccak256(abi_packed)
}

//@todo does nil_sig signature necessary for us? -> a little optimisatin. how about verifying every signature and incrementing cumulative power only for valid signatures.
pub fn check_validator_signatures(
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
    digest: FixedBytes<32>,
    power_threshold: U256,
) -> bool {
    let mut cumulative_power = U256::from(0);
    for i in 0..current_validators.len() {
        if is_sig_nil(&signatures[i]) {
            continue;
        }

        if !signature::verify_sig(current_validators[i].addr, digest.into(), &signatures[i]) {
            continue; // TODO: should revert? -> no if cum_power < powr_threshold, return false
        }
        cumulative_power += current_validators[i].power;

        if cumulative_power >= power_threshold {
            break;
        }
    }
    if cumulative_power < power_threshold {
        return false;
    }
    true
}

//@todo if representation of the sol! generated structs are not FFI safe, then abi pack & write to memory and abi decode to indidvidual structs.
// #[no_mangle]
// pub unsafe extern "C" fn update_validator_set(new_nonce: U256, validator_set_nonce: U256, data_root_tuple_nonce_ptr: *const B32,validator_ptr: *const Vec<Validator>, signature_ptr: *const Vec<Signature>) -> bool{
//     // to enable support for any contrat format we should follow a generic way of giving only a pointer -> and unpacking rest of the data using that pointer. -> need to be more careful with custom implementation made with runtime side & rust-contract interface side, to make it more generic
//     let validators = unsafe { &*validator_ptr };
//     let signatures = unsafe { &*signature_ptr};
//     let data_root_tuple_nonce = unsafe { &*data_root_tuple_nonce_ptr};
//     SolArrayOf::<Validator>::abi_encode(validators);
//     true
// }

pub fn update_validator_set(
    new_nonce: U256,
    old_nonce: U256,
    new_power_threshold: U256,
    new_validator_set_hash: FixedBytes<32>,
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
) -> bool {
    let one = U256::from(1);
    // @todo dummy data
    let current_nonce = one;
    let current_power_threshold = U256::from(3333);
    let last_validator_set_check_point =
        fixed_bytes!("4a5cc92ce4a0fb368c83da44ea489e4b908ce75bdc460c31c662f35fd3911ff1");
    if new_nonce != current_nonce + one {
        return false;
    }
    if current_validators.len() != signatures.len() {
        return false;
    }
    let current_validator_set_hash = compute_validator_set_hash(&current_validators);

    if domain_seperator_validator_set_hash(
        old_nonce,
        current_power_threshold,
        current_validator_set_hash,
    ) != last_validator_set_check_point
    {
        return false;
    }
    let new_check_point =
        domain_seperator_validator_set_hash(new_nonce, new_power_threshold, new_validator_set_hash);
    let status = check_validator_signatures(
        current_validators,
        signatures,
        new_check_point,
        current_power_threshold,
    );
    if status != true {
        return false;
    }
    // @todo update state variables
    true
}

// this shares the same checks as update_validator_set(), only change is update state variables
fn submit_data_root_tuple_root(
    new_nonce: U256,
    validator_set_nonce: U256,
    data_root_tuple_root: FixedBytes<32>, // should this be named as B256?
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
) -> bool {
    let one = U256::from(1);
    // @todo dummy data
    let current_nonce = one;
    let current_power_threshold = U256::from(3333);
    let last_validator_set_check_point =
        fixed_bytes!("4a5cc92ce4a0fb368c83da44ea489e4b908ce75bdc460c31c662f35fd3911ff1");
    if new_nonce != current_nonce + one {
        return false;
    }
    if current_validators.len() != signatures.len() {
        return false;
    }
    let current_validator_set_hash = compute_validator_set_hash(&current_validators);

    if domain_seperator_validator_set_hash(
        validator_set_nonce,
        current_power_threshold,
        current_validator_set_hash,
    ) != last_validator_set_check_point
    {
        return false;
    }
    let c = domain_seperator_data_root_tuple_root(new_nonce, data_root_tuple_root);
    let status =
        check_validator_signatures(current_validators, signatures, c, current_power_threshold);
    if status != true {
        return false;
    }
    //@todo update state variables
    true
}

fn verify_attestation(
    tuple_root_nonce: U256,
    tuple: DataRootTuple,
    proof: BinaryMerkleProof,
) -> bool {
    //@todo dummy data
    let state_event_nonce = U256::from(2);
    let mut state_data_tuple_roots: HashMap<U256, FixedBytes<32>> = HashMap::new();
    state_data_tuple_roots.insert(
        tuple_root_nonce,
        fixed_bytes!("82dc1607d84557d3579ce602a45f5872e821c36dbda7ec926dfa17ebc8d5c013"),
    );
    if tuple_root_nonce > state_event_nonce {
        return false;
    }
    let root = state_data_tuple_roots[&tuple_root_nonce];
    let is_valid_proof = binary_merkle_tree::verify(root, proof, tuple.abi_encode().into());

    is_valid_proof
}
#[cfg(test)]
mod tests {
    use alloy_primitives::{address, fixed_bytes, hex::FromHex, Address, FixedBytes, B256, U256};

    use crate::{
        submit_data_root_tuple_root, update_validator_set, verify_attestation, BinaryMerkleProof,
        DataRootTuple, Signature,
    };

    use super::{
        compute_validator_set_hash, domain_seperator_data_root_tuple_root,
        domain_seperator_validator_set_hash, Validator,
    };

    // all values used in the test(s) are from blobstream.t.sol;

    fn get_dummy_validators_one() -> Vec<Validator> {
        let pub_addr_1 = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        let power_1 = U256::from(5000);
        vec![Validator {
            addr: pub_addr_1,
            power: power_1,
        }]
    }
    #[test]
    fn test_address_conversion_sol_type() {
        let pub_addr_1 = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        assert_eq!(
            pub_addr_1.to_checksum(None),
            "0x9c2B12b5a07FC6D719Ed7646e5041A7E85758329",
        )
    }
    #[test]
    fn test_compute_validator_set_hash() {
        let validator = get_dummy_validators_one();
        let computed_validator_set_hash = compute_validator_set_hash(&validator);
        let validator_set_hash =
            fixed_bytes!("5b192b99215c5d34e61b406e8c150c54a0c58416e61a5c49e61625bad3e0f123");
        assert_eq!(validator_set_hash, computed_validator_set_hash)
    }

    #[test]
    fn test_domain_seperator_validator_set_hash() {
        let nonce = U256::from(3);
        let power_threshold = U256::from(3333);
        let validator_set_hash = compute_validator_set_hash(&get_dummy_validators_one());
        let computed_dsvsh =
            domain_seperator_validator_set_hash(nonce, power_threshold, validator_set_hash);
        let dsvsh =
            fixed_bytes!("d1f777271a9354401e3ec0aa8a8b697cdfd9fc2a68690accb00f63c15892e3cc");
        assert_eq!(computed_dsvsh, dsvsh)
    }

    #[test]
    fn test_domain_seperator_data_root_tuple_root() {
        let nonce = U256::from(2);
        let data_root_tuple_root =
            B256::from_hex("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e")
                .unwrap();

        let computed_dsdrtr = domain_seperator_data_root_tuple_root(nonce, data_root_tuple_root);
        let dsdrtr =
            fixed_bytes!("7c8dcea15d58179551c512f82d59037049bd1cd9b82b2c59a997e75997959d17");
        assert_eq!(computed_dsdrtr, dsdrtr)
    }

    #[test]
    fn test_update_validator_set() {
        // @todo contract system is stateless(now), dummy values defined in the function updateValidatorSet to match the statevalues
        let initial_val_set_nonce = U256::from(1);
        let mut validators = get_dummy_validators_one();
        let new_nonce = U256::from(2);
        validators.push(Validator {
            addr: address!("e650B084f05C6194f6e552e3b9f08718Bc8a9d56"),
            power: U256::from(5000),
        });
        let voting_power = U256::from(10_000);
        let new_power_threshold = U256::from(2) * voting_power / U256::from(3); //6_666
        let new_validator_set_hash = compute_validator_set_hash(&validators);
        let current_validators = get_dummy_validators_one();
        let sig = Signature {
            v: 27,
            r: fixed_bytes!("02bd9e5fe41ca09e69c688eb127ba3a710ba0f9f9080b13c1f003126a74be2d5"),
            s: fixed_bytes!("6dc6943fc93d17984e3ac3023b15030b33a5c9b6e647ddfb3a7f19a1c3ce9a2e"),
        };
        let sigs = vec![sig];
        assert_eq!(
            true,
            update_validator_set(
                new_nonce,
                initial_val_set_nonce,
                new_power_threshold,
                new_validator_set_hash,
                current_validators,
                sigs,
            )
        );
    }

    #[test]
    fn test_submit_data_root_tuple_root() {
        let initial_val_set_nonce = U256::from(1);
        let nonce = U256::from(2);
        let new_tuple_root =
            fixed_bytes!("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e");
        let validators = get_dummy_validators_one();
        let sig = vec![Signature {
            v: 28,
            r: fixed_bytes!("f48f949c827fb5a0db3bf416ea657d2750eeadb7b6906c6fb857d2fd1dd57181"),
            s: fixed_bytes!("46ae888d1453fd5693b0148cecf0368b42552e597a3b628456946cf63b627b04"),
        }];
        assert_eq!(
            true,
            submit_data_root_tuple_root(
                nonce,
                initial_val_set_nonce,
                new_tuple_root,
                validators,
                sig,
            )
        );
    }
    #[test]
    fn test_verify_attestations() {
        let initial_val_set_nonce = U256::from(1);
        let nonce = U256::from(3);
        let new_tuple_root =
            fixed_bytes!("82dc1607d84557d3579ce602a45f5872e821c36dbda7ec926dfa17ebc8d5c013");
        let new_tuple =
            fixed_bytes!("0101010101010101010101010101010101010101010101010101010101010101");
        let height = U256::from(1);
        let mut side_nodes = Vec::new();
        side_nodes.push(fixed_bytes!(
            "98ce42deef51d40269d542f5314bef2c7468d401ad5d85168bfab4c0108f75f7"
        ));
        side_nodes.push(fixed_bytes!(
            "575664048c9e64260eca2304d177b11d1566d0c954f1417fc76a4f9f27350063"
        ));
        let new_tuple_proof = BinaryMerkleProof {
            sideNodes: side_nodes,
            key: U256::from(1),
            numLeaves: U256::from(4),
        };
        let tuple = DataRootTuple {
            height: height,
            dataRoot: new_tuple,
        };
        assert_eq!(true, verify_attestation(nonce, tuple, new_tuple_proof))
    }
}

//@todo alloy-primitives got many useful macros
//@todo test these functions with same values for outputs in soldity as a final confirmation

//@todo https://docs.rs/alloy-sol-macro/0.6.3/alloy_sol_macro/macro.sol.html#functions-and-errors -> function layout implementations for us. simple abi packed struct for usage & a wrapper around this?
