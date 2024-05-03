#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub mod input_type;
// alloy imports
pub use alloy_primitives::{
    fixed_bytes, hex::FromHex, keccak256, Bytes, FixedBytes, Uint, B256, U256, U64,
};
use alloy_sol_macro::sol;
use alloy_sol_types::{SolType, SolValue};
use input_type::OutputBreaker;
// std lib

// seq wasm sdk
pub use seq_wasm_sdk::allocator::*;
use seq_wasm_sdk::slice;
use seq_wasm_sdk::state;
use seq_wasm_sdk::utils::TxContext;

use input_type::{CommitHeaderRangeInput, InitializerInput, InputHashPacker, UpdateFreezeInput};
const STATIC_ISINITIALIZED: u32 = 0;
const STATIC_FROZEN: u32 = 1;
const STATIC_OWNER: u32 = 2;
const STATIC_LATESTBLOCK: u32 = 3;
const STATIC_LATEST_AUTHORITY_SET_ID: u32 = 4;
const STATIC_HEADER_RANGE_FUNCTION_ID: u32 = 5;
const STATIC_ROTATE_FUNCTION_ID: u32 = 6;

const DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH: u32 = 2;
const DYNAMIC_AUTHORITY_SET_ID_TO_HASH: u32 = 3;
const DYAMIC_DATA_ROOT_COMMITMENTS: u32 = 4;
const DYNAMIC_STATE_ROOT_COMMITMENTS: u32 = 5;
const DYNAMIC_RANGE_START_BLOCKS: u32 = 6;

const HEADER_RANGE_FUNCTION_ID: FixedBytes<32> =
    //@todo this is dummy header range funciton id.
    fixed_bytes!("16cb5c45290c8545b9998275c07e7577fa0962bb6e35597c69de570649b7083f");

#[cfg_attr(all(target_arch = "wasm32"), export_name = "initializer")]
#[no_mangle]
pub extern "C" fn initializer(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if is_initialized() {
        // contract already initialized
        return false;
    }
    let msg_sender = msg_sender(tx_context);
    let (
        height,
        header,
        authority_set_id,
        authority_set_hash,
        header_range_function_id,
        rotate_function_id,
    ) = InitializerInput::new(ptr, len).unpack();
    state::store_mapping_u32_bytes32(DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH, 2, header);
    state::store_mapping_u64_bytes32(DYNAMIC_AUTHORITY_SET_ID_TO_HASH, 3, authority_set_hash);
    state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, authority_set_id);
    state::store_u32(STATIC_LATESTBLOCK, height);
    state::store_bytes32(STATIC_HEADER_RANGE_FUNCTION_ID, header_range_function_id);
    state::store_bytes32(STATIC_ROTATE_FUNCTION_ID, rotate_function_id);
    state::store_vec(STATIC_OWNER, &msg_sender);
    state::store_bool(STATIC_ISINITIALIZED, 1);
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_freeze")]
#[no_mangle]
pub extern "C" fn update_freeze(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if !is_initialized() {
        return false; //this may not be necessary because owner is no one if not initialized
    }
    let msg_sender = msg_sender(tx_context);
    let owner = state::get_vec(STATIC_OWNER);
    if msg_sender != owner {
        // not an owner
        return false;
    }
    let freeze = UpdateFreezeInput::new(ptr, len).freeze;
    state::store_bool(STATIC_FROZEN, freeze as u32);
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "commit_header_range")]
#[no_mangle]
pub unsafe extern "C" fn commit_header_range(
    _: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    if is_frozen() && !is_initialized() {
        return false;
    }
    let (authority_set_id, target_block, input, output, _proof) =
        CommitHeaderRangeInput::new(ptr, len).unpack();
    let trusted_block = state::get_u32(STATIC_LATESTBLOCK);
    let latest_authority_set_id = state::get_u64(STATIC_LATEST_AUTHORITY_SET_ID);
    let trusted_header =
        state::get_mapping_u32_bytes32(DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH, trusted_block);
    let authority_set_hash =
        state::get_mapping_u64_bytes32(DYNAMIC_AUTHORITY_SET_ID_TO_HASH, authority_set_id);
    if trusted_header == FixedBytes::<32>::new([0; 32])
        || authority_set_hash == FixedBytes::<32>::new([0; 32])
    {
        // trusted header not found or authority set hash not found
        return false;
    }
    if authority_set_id < latest_authority_set_id {
        // old set id
        return false;
    }
    if authority_set_id > latest_authority_set_id {
        // new set id, but is this necessary @todo
        state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, authority_set_id);
    }
    if target_block < trusted_block {
        // old block
        return false;
    }
    let packed_input = InputHashPacker {
        latestBlock: trusted_block,
        trustedHeader: trusted_header,
        authoritySetId: authority_set_id,
        authoritySetHash: authority_set_hash,
        targetBlock: target_block,
    }
    .abi_encode_packed();
    if packed_input != input {
        // proof built on a wrong block
        return false;
    }
    if state::gnark_verify(trusted_block as u64, HEADER_RANGE_FUNCTION_ID) {
        let (target_header_hash, state_root_commitment, data_root_commitment) =
            OutputBreaker::decode(&output);
        // keccak hash & state update
    }
    true
}
fn is_frozen() -> bool {
    if state::get_bool(STATIC_FROZEN) == 1 {
        true
    } else {
        false
    }
}

fn is_initialized() -> bool {
    if state::get_bool(STATIC_ISINITIALIZED) == 1 {
        true
    } else {
        false
    }
}

fn msg_sender(tx_context: *const TxContext) -> Vec<u8> {
    let tx_context = unsafe { &*tx_context };
    tx_context.msg_sender()
}
