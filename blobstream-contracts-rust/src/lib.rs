#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

pub mod binary_merkle_tree;
pub mod input_type;

// crate imports
use input_type::{
    BinaryMerkleProof, CommitHeaderRangeInput, InitializerInput, InputHashPacker, LeafDigestPacker,
    NodeDigestPacker, OutputBreaker, UpdateFreezeInput, VAInput,
};

// seq wasm sdk
pub use seq_wasm_sdk::allocator::*;
use seq_wasm_sdk::slice;
use seq_wasm_sdk::state;
use seq_wasm_sdk::utils::TxContext;
use seq_wasm_sdk::{fixed_bytes, sol, Bytes, FixedBytes, FromHex, SolType, SolValue, U256};

// get state variables enum from program vm.
const STATIC_ISINITIALIZED: u32 = 0;
const STATIC_FROZEN: u32 = 1;
const STATIC_OWNER: u32 = 2;
const STATIC_LATESTBLOCK: u32 = 3;
const STATIC_STATE_PROOFNONCE: u32 = 4;

// the below represnted values act as an offset. we need to make sure collisions will not happen
// tune with the offsets
const DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH: u32 = 2;
const DYNAMIC_STATE_DATA_COMMITMENTS: u32 = 3;

// CONSTANT VARIABLES
const DATA_COMMITMENT_MAX: u64 = 10_000;
const HEADER_RANGE_FUNCTION_ID: FixedBytes<32> =
    fixed_bytes!("16cb5c45290c8545b9998275c07e7577fa0962bb6e35597c69de570649b7083f");

#[cfg_attr(all(target_arch = "wasm32"), export_name = "initializer")]
#[no_mangle]
pub extern "C" fn initializer(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if is_initialized() {
        // contract already initialized
        return false;
    }
    let msg_sender = msg_sender(tx_context);
    let (height, header) = InitializerInput::new(ptr, len).unpack();
    state::store_u64(STATIC_LATESTBLOCK, height);
    state::store_mapping_u64_bytes32(DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH, height, header);
    state::store_u256(STATIC_STATE_PROOFNONCE, U256::from(1));
    state::store_vec(STATIC_OWNER, &msg_sender);
    state::store_bool(STATIC_ISINITIALIZED, 1);
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_freeze")]
#[no_mangle]
pub extern "C" fn update_freeze(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
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
    let (target_block, input, output, _proof) = CommitHeaderRangeInput::new(ptr, len).unpack();
    let trusted_block = state::get_u64(STATIC_LATESTBLOCK);
    let trusted_header =
        state::get_mapping_u64_bytes32(DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH, trusted_block);
    let proof_nonce = state::get_u256(STATIC_STATE_PROOFNONCE);
    if trusted_header == FixedBytes::<32>::new([0; 32]) {
        return false;
    }
    let packed_input = InputHashPacker {
        latest_block: trusted_block,
        trusted_header,
        target_block,
    }
    .abi_encode_packed();
    if packed_input != input {
        // proof built on a wrong block
        return false;
    }
    // checking invarients before verifying the proof
    if target_block <= trusted_block || target_block - trusted_block > DATA_COMMITMENT_MAX {
        return false;
    }
    if state::gnark_verify(HEADER_RANGE_FUNCTION_ID) {
        // valid proof
        let (target_header, data_commitment) = OutputBreaker::decode(&output);
        state::store_mapping_u64_bytes32(
            DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH,
            target_block,
            target_header,
        );
        state::store_mapping_u256_bytes32(
            DYNAMIC_STATE_DATA_COMMITMENTS,
            proof_nonce,
            data_commitment,
        );
        state::store_u256(STATIC_STATE_PROOFNONCE, proof_nonce + U256::from(1));
        state::store_u64(STATIC_LATESTBLOCK, target_block);
        true
    } else {
        // invalid proof
        false
    }
}

/// Verify the attestation for the given proof nonce, tuple, and proof. This is taken from
/// the existing Blobstream contract and is used to verify the data hash for a specific block
/// against a posted data commitment.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "verify_attestation")]
#[no_mangle]
pub extern "C" fn verify_attestation(_: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if is_frozen() && is_initialized() {
        return false;
    }

    let (proof_nonce, tuple, proof) = VAInput::new(ptr, len).unpack();

    let state_proof_nonce = state::get_u256(STATIC_STATE_PROOFNONCE);
    if proof_nonce > state_proof_nonce {
        return false;
    }
    let root = state::get_mapping_u256_bytes32(DYNAMIC_STATE_DATA_COMMITMENTS, proof_nonce);
    let is_valid_proof = binary_merkle_tree::verify(root, proof, tuple.abi_encode().into());

    is_valid_proof
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
