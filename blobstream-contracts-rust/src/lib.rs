#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub mod signature;

// standard library
// use std::collections::HashMap;
// static allocator
// use std::alloc::{alloc, Layout};
use alloc::vec::Vec;
use std::mem::MaybeUninit;
use std::str::FromStr;
// use alloy_primitives::utils::try_collect_vec;
// alloy
use alloy_primitives::{
    /*fixed_bytes, hex,*/ bytes, hex::FromHex, keccak256, FixedBytes, Signature, B256, U256,
};
use alloy_sol_macro::sol;
use alloy_sol_types::SolType;
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

    struct Signature2 {
        uint8 v;
        bytes32 r;
        bytes32 s;
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
        bytes32 hash;
    }
}

// type B32 = sol!{bytes32}; -> dont use this, this gives the same functionality as B256,but causes issues with its declaration of FixedBytes<32>
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
    let packed = PackerDomainSeperatorValidator {
        VALIDATOR_SET_HASH_DOMAIN_SEPARATOR: validator_set_hash_domain_seperator,
        nonce: nonce,
        powerThreshold: power_threshold,
        validatorSetHash: validator_set_hash,
    };
    let abi_packed = PackerDomainSeperatorValidator::abi_encode(&packed);
    keccak256(abi_packed)
}

pub fn domain_seperator_data_root_tuple_root(
    nonce: U256,
    data_root_tuple_root: FixedBytes<32>,
) -> FixedBytes<32> {
    let data_root_tuple_root_domain_seperator =
        B256::from_hex("7472616e73616374696f6e426174636800000000000000000000000000000000").unwrap();
    let packed = PackerDomainSeperatorDataRootTuple {
        DATA_ROOT_TUPLE_ROOT_DOMAIN_SEPERATOR: data_root_tuple_root_domain_seperator,
        nonce: nonce,
        dataRootTupleRoot: data_root_tuple_root,
    };
    let abi_packed = PackerDomainSeperatorDataRootTuple::abi_encode(&packed);
    keccak256(abi_packed)
}

//@todo does nil_sig signature necessary for us? how about verifying every signature and incrementing cumulative power only for valid signatures.
pub fn check_validator_signatures(
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
    digest: FixedBytes<32>,
    power_threshold: U256,
) -> bool {
    // building it for testing poc, TODO: add compatibilty to wasm
    let mut cumulative_power = U256::from(0);
    // can also use .enumerate.into_iter()
    for i in 0..current_validators.len() {
        if is_sig_nil(signatures[i]) {
            continue;
        }

        if !signature::verify_sig(current_validators[i].addr, digest, signatures[i]) {
            continue; // TODO: should revert?
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
//  @todo Layout followed
// Update Validator set
// Submit Dataroot tuple
// Verify attestation

// should use same encoding everywhere -> assuming relayer gets the data packed according to solidity abi

// for now we are translating solidity contracts to rust -> as is

//@todo if representation of the sol! generated structs are not FFI safe, then abi pack & write to memory and abi decode to indidvidual structs.
// #[no_mangle]
// pub unsafe extern "C" fn update_validator_set(new_nonce: U256, validator_set_nonce: U256, data_root_tuple_nonce_ptr: *const B32,validator_ptr: *const Vec<Validator>, signature_ptr: *const Vec<Signature2>) -> bool{
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
    let current_nonce = U256::from(0);
    let current_power_threshold = U256::from(100);
    let last_validator_set_check_point = FixedBytes::new([0; 32]);
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

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, hex::FromHex, Address, B256, U256};

    use super::{
        compute_validator_set_hash, domain_seperator_data_root_tuple_root,
        domain_seperator_validator_set_hash, Validator,
    };
    fn get_dummy_validators_one() -> Vec<Validator> {
        let pub_key: &[u8] = &[
            53 as u8, 174, 235, 28, 172, 17, 215, 192, 132, 230, 95, 50, 23, 241, 17, 218, 3, 73,
            59, 177,
        ];
        let pub_addr_1 = address!("35AeEb1cAc11D7C084e65F3217f111dA03493bB1");
        let power_1 = U256::from(30);
        vec![Validator {
            addr: pub_addr_1,
            power: power_1,
        }]
    }
    #[test]
    fn test_compute_validator_set_hash() {
        let validator = get_dummy_validators_one();
        println!("{:?}", compute_validator_set_hash(&validator));
    }

    #[test]
    fn test_domain_seperator_validator_set_hash() {
        let nonce = U256::from(3);
        let power_threshold = U256::from(3000);
        let validator_set_hash = compute_validator_set_hash(&get_dummy_validators_one());
        println!(
            "{:?}",
            domain_seperator_validator_set_hash(nonce, power_threshold, validator_set_hash)
        );
    }

    #[test]
    fn test_domain_seperator_data_root_tuple_root() {
        let nonce = U256::from(3);
        let data_root_tuple_root =
            B256::from_hex("d149f160dec348d8582b8c2629c91fab8189b8dca205c4c01bb378f2f5450c3b")
                .unwrap();
        println!(
            "{:?}",
            domain_seperator_data_root_tuple_root(nonce, data_root_tuple_root)
        )
    }
}

//@todo alloy-primitives got many useful macros
//@todo test these functions with same values for outputs in soldity as a final confirmation
