#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub mod binary_merkle_tree;
pub mod input_type;
pub mod state;

// static allocator
use alloc::vec::Vec;

// alloy imports
pub use alloy_primitives::{fixed_bytes, hex::FromHex, keccak256, Bytes, FixedBytes, B256, U256};
use alloy_sol_macro::sol;
use alloy_sol_types::{SolType, SolValue};

// std lib
use core::slice;
use input_type::{InitializerInput, VAInput};
pub use std::alloc::{alloc, Layout};
use std::mem::MaybeUninit;

// solidity type decleration begin ----
sol! {
    struct DataRootTuple{
        uint256 height;
        bytes32 dataRoot;
    }
    struct BinaryMerkleProof{
        bytes32[] sideNodes;
        uint256 key;
        uint256 numLeaves;
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

// type SolArrayOf<T> = sol! { T[] };

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
//@todo state variables will change.
const STATIC_STATE_LASTVALIDATORSETCHECKPOINT: u32 = 0;
const STATIC_STATE_POWERTHRESHOLD: u32 = 1;
const STATIC_STATE_EVENTNONCE: u32 = 2;

const DYNAMIC_STATE_DATAROOTTUPLEROOTS: u32 = 2;

//@todo how do we do the initializer?
#[no_mangle]
pub extern "C" fn initializer(ptr: *const u8, len: u32) -> bool {
    //@todo need to change intializer implementation
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
pub extern "C" fn verify_attestation(ptr: *const u8, len: u32) -> bool {
    let (tuple_root_nonce, tuple, proof) = VAInput::new(ptr, len).unpack();

    let state_event_nonce = state::get_u256(STATIC_STATE_EVENTNONCE);
    if tuple_root_nonce > state_event_nonce {
        return false;
    }
    let root = /*state_data_tuple_roots[&tuple_root_nonce];*/ state::get_mapping_u256_bytes32(DYNAMIC_STATE_DATAROOTTUPLEROOTS, tuple_root_nonce);
    let is_valid_proof = binary_merkle_tree::verify(root, proof, tuple.abi_encode().into());

    is_valid_proof
}

//@todo alloy-primitives got many useful macros
//@todo https://docs.rs/alloy-sol-macro/0.6.3/alloy_sol_macro/macro.sol.html#functions-and-errors -> function layout implementations for us. simple abi packed struct for usage & a wrapper around this?
