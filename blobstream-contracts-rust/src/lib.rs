#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

pub mod binary_merkle_tree;
pub mod input_type;

// crate imports.
use input_type::{
    BinaryMerkleProof, CommitHeaderRangeInput, InitializerInput, LeafDigestPacker,
    NodeDigestPacker, UpdateFreezeInput, UpdateGenesisStateInput, UpdateProgramVkeyInput, VAInput,
};

// seq wasm sdk imports.
pub use seq_wasm_sdk::allocator::*;
use seq_wasm_sdk::{precompiles, state, types, utils::TxContext};
use seq_wasm_sdk::{slice, sol, Bytes, FixedBytes, FromHex, SolType, SolValue, U256};

// get state variables enum from program vm.
const STATIC_ISINITIALIZED: u32 = 0;
const STATIC_FROZEN: u32 = 1;
const STATIC_GUARDIAN: u32 = 2;
const STATIC_LATESTBLOCK: u32 = 3;
const STATIC_STATE_PROOFNONCE: u32 = 4;
const STATIC_BLOBSTREAM_PROGRAM_VKEY_HASH: u32 = 5; // sha256 hash of the verification key.
const STATIC_BLOBSTREAM_PROGRAM_VKEY: u32 = 6; // actual verification key.

// ids for storing dynamic variables.
const MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID: u32 = 1;
const MAPPING_STATE_DATA_COMMITMENTS_ID: u32 = 2;

// CONSTANT VARIABLES
const DATA_COMMITMENT_MAX: u64 = 1_000;

#[cfg_attr(all(target_arch = "wasm32"), export_name = "initializer")]
#[no_mangle]
pub extern "C" fn initializer(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if is_initialized() {
        // contract already initialized
        return false;
    }

    // Decode msg_sender from tx_context and inputs from IntializerInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (height, header, blobstream_program_vkey_hash, blobstream_program_vkey) =
        InitializerInput::new(ptr, len).unpack();

    // Store the initial state variables and set contract as initialized.
    state::store_u64(STATIC_LATESTBLOCK, height);
    state::store_mapping_u64_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, height, header);
    state::store_u256(STATIC_STATE_PROOFNONCE, U256::from(1));
    state::store_address(STATIC_GUARDIAN, &msg_sender);
    state::store_vec(
        STATIC_BLOBSTREAM_PROGRAM_VKEY_HASH,
        &blobstream_program_vkey_hash,
    );
    state::store_vec(STATIC_BLOBSTREAM_PROGRAM_VKEY, &blobstream_program_vkey);
    state::store_bool(STATIC_ISINITIALIZED, 1);

    // Call executed without any errors, return true.
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_freeze")]
#[no_mangle]
pub extern "C" fn update_freeze(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateFreezeInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let freeze = UpdateFreezeInput::new(ptr, len).freeze;

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN).try_into().unwrap();
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update the freeze state variable.
    state::store_bool(STATIC_FROZEN, freeze as u32);

    // Call executed without any errors, return true.
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_genesis_state")]
#[no_mangle]
pub extern "C" fn update_genesis_state(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateGenesisStateInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (height, header) = UpdateGenesisStateInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update the genesis state variables.
    state::store_mapping_u64_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, height, header);
    state::store_u64(STATIC_LATESTBLOCK, height);

    // Call executed without any errors, return true.
    true
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_program_vkey")]
#[no_mangle]
pub extern "C" fn update_program_vkey(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateGenesisStateInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (program_vkey_hash, program_vkey) = UpdateProgramVkeyInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update program vkey.
    state::store_vec(STATIC_BLOBSTREAM_PROGRAM_VKEY_HASH, &program_vkey_hash);
    state::store_vec(STATIC_BLOBSTREAM_PROGRAM_VKEY, &program_vkey);

    // Call executed without any errors, return true.
    true
}

/// Commits the new header at targetBlock and the data commitment for the block range [latestBlock, targetBlock].
#[cfg_attr(all(target_arch = "wasm32"), export_name = "commit_header_range")]
#[no_mangle]
pub extern "C" fn commit_header_range(_: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // unpack proof and public values from CommitHeaderRangeInput.
    let (proof, public_values) = CommitHeaderRangeInput::new(ptr, len).unpack();
    // if contract is frozen or not initialized, return false.
    if is_frozen() || !is_initialized() {
        return false;
    }
    let (
        po_trusted_header_hash,
        target_header_hash,
        data_commitment,
        po_trusted_block,
        target_block,
        _,
    ) = CommitHeaderRangeInput::new(ptr, len).unpack_po();

    // fetch the latest block and trusted header hash from the state.
    let latest_block = state::get_u64(STATIC_LATESTBLOCK);
    let trusted_header =
        state::get_mapping_u64_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, latest_block);

    // sanity check public values and state values.
    if trusted_header == FixedBytes::<32>::new([0; 32]) {
        return false;
    }
    if po_trusted_block != latest_block {
        return false;
    }
    if trusted_header != po_trusted_header_hash {
        return false;
    }
    if target_block <= latest_block || target_block - latest_block > DATA_COMMITMENT_MAX {
        return false;
    }

    // fetch blobstream program vkey and program vkey hash from the state.
    let blobstream_program_vkey_hash = state::get_vec(STATIC_BLOBSTREAM_PROGRAM_VKEY_HASH);
    let blobstream_program_vkey = state::get_vec(STATIC_BLOBSTREAM_PROGRAM_VKEY);
    // verify sp1 plonk proof.
    if precompiles::gnark_verify(
        blobstream_program_vkey_hash,
        public_values,
        proof,
        blobstream_program_vkey,
    ) {
        // proof is valid, update state variables.
        let proof_nonce = state::get_u256(STATIC_STATE_PROOFNONCE);

        state::store_mapping_u64_bytes32(
            MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID,
            target_block,
            target_header_hash,
        );
        state::store_mapping_u256_bytes32(
            MAPPING_STATE_DATA_COMMITMENTS_ID,
            proof_nonce,
            data_commitment,
        );
        state::store_u256(STATIC_STATE_PROOFNONCE, proof_nonce + U256::from(1));
        state::store_u64(STATIC_LATESTBLOCK, target_block);

        // Call executed without any errors, return true.
        true
    } else {
        // proof is invalid, return false.
        false
    }
}

/// Verify the attestation for the given proof nonce, tuple, and proof. This is taken from
/// the existing Blobstream contract and is used to verify the data hash for a specific block
/// against a posted data commitment.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "verify_attestation")]
#[no_mangle]
pub extern "C" fn verify_attestation(_: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // If the contract is frozen or not initialized, return false.
    if is_frozen() || !is_initialized() {
        return false;
    }

    // Decode the inputs from the VAInput struct.
    let (proof_nonce, tuple, proof) = VAInput::new(ptr, len).unpack();

    // Fetch the state proof nonce and check if the proof nonce is valid.
    let state_proof_nonce = state::get_u256(STATIC_STATE_PROOFNONCE);
    if proof_nonce > state_proof_nonce || proof_nonce == U256::from(0) {
        return false;
    }

    // Fetch the data commitment from the state and verify the proof.
    let root = state::get_mapping_u256_bytes32(MAPPING_STATE_DATA_COMMITMENTS_ID, proof_nonce);
    let is_proof_valid = binary_merkle_tree::verify(root, proof, tuple.abi_encode().into());

    is_proof_valid
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
