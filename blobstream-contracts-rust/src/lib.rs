#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub mod binary_merkle_tree;
pub mod input_type;
pub mod signature;
pub mod state;

// static allocator
use alloc::vec::Vec;
// standard library
pub use alloy_primitives::{fixed_bytes, hex::FromHex, keccak256, FixedBytes, B256, U256};
use alloy_sol_macro::sol;
use alloy_sol_types::{SolType, SolValue};
use core::slice;

pub use std::alloc::{alloc, Layout};
use std::mem::MaybeUninit;

use input_type::{InitializerInput, SDRTRInput, UVSInput, VAInput};
use signature::is_sig_nil;

// solidity type decleration begin ----
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

// solidity type decleration ends ----

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

// state variables enum
const STATIC_STATE_LASTVALIDATORSETCHECKPOINT: u32 = 0;
const STATIC_STATE_POWERTHRESHOLD: u32 = 1;
const STATIC_STATE_EVENTNONCE: u32 = 2;

const DYNAMIC_STATE_DATAROOTTUPLEROOTS: u32 = 2;

fn compute_validator_set_hash(validator: &Vec<Validator>) -> FixedBytes<32> {
    let tokenized_validator = SolArrayOf::<Validator>::abi_encode(validator);
    keccak256(tokenized_validator)
}

fn domain_seperator_validator_set_hash(
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

fn domain_seperator_data_root_tuple_root(
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

//@todo does nil_sig signature necessary for us? -> a little optimisation. how about verifying every signature and incrementing cumulative power only for valid signatures.
fn check_validator_signatures(
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

//@todo how do we do the initializer?
#[no_mangle]
pub extern "C" fn initializer(ptr: *const u8, len: u32) -> bool {
    let (nonce, power_threshold, validator_set_check_point) =
        InitializerInput::new(ptr, len).unpack();
    state::store_u256(STATIC_STATE_EVENTNONCE, nonce);
    state::store_u256(STATIC_STATE_POWERTHRESHOLD, power_threshold);
    state::store_bytes32(
        STATIC_STATE_LASTVALIDATORSETCHECKPOINT,
        validator_set_check_point,
    );
    true
}

#[no_mangle]
pub extern "C" fn update_validator_set(ptr: *const u8, len: u32) -> bool {
    let one = U256::from(1);

    let (
        new_nonce,
        old_nonce,
        new_power_threshold,
        new_validator_set_hash,
        current_validators,
        signatures,
    ) = UVSInput::new(ptr, len).unpack();

    let current_nonce = state::get_u256(STATIC_STATE_EVENTNONCE);
    let current_power_threshold = state::get_u256(STATIC_STATE_POWERTHRESHOLD);
    let last_validator_set_check_point =
        state::get_bytes32(STATIC_STATE_LASTVALIDATORSETCHECKPOINT);
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

    state::store_bytes32(STATIC_STATE_LASTVALIDATORSETCHECKPOINT, new_check_point);
    state::store_u256(STATIC_STATE_POWERTHRESHOLD, new_power_threshold);
    state::store_u256(STATIC_STATE_EVENTNONCE, new_nonce);
    true
}

#[no_mangle]
pub extern "C" fn submit_data_root_tuple_root(sdrtr_ptr: *const u8, len: u32) -> bool {
    // this shares the same checks as update_validator_set(), only change is update state variables
    let one = U256::from(1);
    let (new_nonce, validator_set_nonce, data_root_tuple_root, current_validators, signatures) =
        SDRTRInput::new(sdrtr_ptr, len).unpack();

    let current_nonce = state::get_u256(STATIC_STATE_EVENTNONCE);
    let current_power_threshold = state::get_u256(STATIC_STATE_POWERTHRESHOLD);
    let last_validator_set_check_point =
        state::get_bytes32(STATIC_STATE_LASTVALIDATORSETCHECKPOINT);
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

    state::store_u256(STATIC_STATE_EVENTNONCE, new_nonce);
    state::store_mapping_u256_bytes32(
        DYNAMIC_STATE_DATAROOTTUPLEROOTS,
        new_nonce,
        data_root_tuple_root,
    );
    true
}

#[no_mangle]
pub extern "C" fn verify_attestation(ptr: *const u8, len: u32) -> bool {
    let (tuple_root_nonce, tuple, proof) = VAInput::new(ptr, len).unpack();

    let state_event_nonce = state::get_u256(STATIC_STATE_EVENTNONCE);
    // let mut state_data_tuple_roots: HashMap<U256, FixedBytes<32>> = HashMap::new();
    // state_data_tuple_roots.insert(
    //     tuple_root_nonce,
    //     fixed_bytes!("82dc1607d84557d3579ce602a45f5872e821c36dbda7ec926dfa17ebc8d5c013"),
    // );

    if tuple_root_nonce > state_event_nonce {
        return false;
    }
    let root = /*state_data_tuple_roots[&tuple_root_nonce];*/ state::get_mapping_u256_bytes32(DYNAMIC_STATE_DATAROOTTUPLEROOTS, tuple_root_nonce);
    let is_valid_proof = binary_merkle_tree::verify(root, proof, tuple.abi_encode().into());

    is_valid_proof
}

//@todo alloy-primitives got many useful macros
//@todo https://docs.rs/alloy-sol-macro/0.6.3/alloy_sol_macro/macro.sol.html#functions-and-errors -> function layout implementations for us. simple abi packed struct for usage & a wrapper around this?
