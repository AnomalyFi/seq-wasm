#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
pub mod input_type;

// crate imports.
use input_type::{
    CommitHeaderRangeAndRotateInput, HeaderRangeOutputs, InitializerInput, KeyPacker, ProofType,
    RotateOutputs, UpdateBlockRangeDataInput, UpdateCommitmentTreeSizeInput, UpdateFreezeInput,
    UpdateGenesisStateInput, UpdateVectorProgramVkeyInput,
};

// seq wasm sdk
pub use seq_wasm_sdk::allocator::*;
use seq_wasm_sdk::{keccak256, slice, sol, FixedBytes, SolType, SolValue};
use seq_wasm_sdk::{precompiles, state, utils::TxContext};

// state variables
const STATIC_ISINITIALIZED: u32 = 0;
const STATIC_FROZEN: u32 = 1;
const STATIC_GUARDIAN: u32 = 2;
const STATIC_LATESTBLOCK: u32 = 3;
const STATIC_LATEST_AUTHORITY_SET_ID: u32 = 4;
const STATIC_HEADER_RANGE_COMMITMENT_TREE_SIZE: u32 = 5;
const STATIC_VECTOR_PROGRAM_VKEY_HASH: u32 = 6; // hash of vk produced for vectorx ELF.
const STATIC_VECTOR_PROGRAM_VKEY: u32 = 7; // actual verification key.

// ids for storing dynamic variables.
const MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID: u32 = 1;
const MAPPING_AUTHORITY_SET_ID_TO_HASH_ID: u32 = 2;
const MAPPING_DATA_ROOT_COMMITMENTS_ID: u32 = 3;
const MAPPING_STATE_ROOT_COMMITMENTS_ID: u32 = 4;
const MAPPING_RANGE_START_BLOCKS_ID: u32 = 5;

/// This function initializes the contract with the initial state variables.
/// This function can only be called once.
/// Called during the contract deployment.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "initializer")]
#[no_mangle]
pub extern "C" fn initializer(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    if is_initialized() {
        // contract already initialized
        return false;
    }

    // Decode msg_sender from tx_context and inputs from IntializerInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (
        height,
        header,
        authority_set_id,
        authority_set_hash,
        header_range_commitment_tree_size,
        vector_program_vkey_hash,
        vector_program_vkey,
    ) = InitializerInput::new(ptr, len).unpack();

    // Store the initial state variables and set contract as initialized.
    state::store_mapping_u32_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, height, header);
    state::store_mapping_u64_bytes32(
        MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
        authority_set_id,
        authority_set_hash,
    );
    state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, authority_set_id);
    state::store_u32(STATIC_LATESTBLOCK, height);
    state::store_vec(STATIC_VECTOR_PROGRAM_VKEY_HASH, &vector_program_vkey_hash);
    state::store_vec(STATIC_VECTOR_PROGRAM_VKEY, &vector_program_vkey);
    state::store_u32(
        STATIC_HEADER_RANGE_COMMITMENT_TREE_SIZE,
        header_range_commitment_tree_size,
    );

    state::store_address(STATIC_GUARDIAN, &msg_sender);
    state::store_bool(STATIC_ISINITIALIZED, 1);

    // Call executed without any errors, return true.
    true
}

/// Only the guardian can set the contract to a frozen state.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_freeze")]
#[no_mangle]
pub extern "C" fn update_freeze(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateFreezeInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let owner = state::get_address(STATIC_GUARDIAN);
    if msg_sender != owner {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update the freeze state variable.
    let freeze = UpdateFreezeInput::new(ptr, len).freeze;
    state::store_bool(STATIC_FROZEN, freeze as u32);

    // Call executed without any errors, return true.
    true
}

/// Only the guardian can update the program vkey.
#[cfg_attr(
    all(target_arch = "wasm32"),
    export_name = "update_vector_program_vkey"
)]
#[no_mangle]
pub extern "C" fn update_vector_program_vkey(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateVectorProgramVKeyInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (program_vkey_hash, program_vkey) = UpdateVectorProgramVkeyInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update program vkey.
    state::store_vec(STATIC_VECTOR_PROGRAM_VKEY_HASH, &program_vkey_hash);
    state::store_vec(STATIC_VECTOR_PROGRAM_VKEY, &program_vkey);

    // Call executed without any errors, return true.
    true
}

#[cfg_attr(
    all(target_arch = "wasm32"),
    export_name = "update_commitment_tree_size"
)]
#[no_mangle]
pub extern "C" fn update_commitment_tree_size(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateCommitmentTreeSizeInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let header_range_commitment_tree_size = UpdateCommitmentTreeSizeInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the gaurdian, update the commitment tree size.
    state::store_u32(
        STATIC_HEADER_RANGE_COMMITMENT_TREE_SIZE,
        header_range_commitment_tree_size,
    );

    // Call executed without any errors, return true.
    true
}

/// Only the gaurdian can update the genesis state of the contract.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_genesis_state")]
#[no_mangle]
pub extern "C" fn update_genesis_state(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateGenesisStateInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (height, header, authority_set_id, authority_set_hash) =
        UpdateGenesisStateInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian, update the genesis state variables.
    state::store_mapping_u32_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, height, header);
    state::store_u32(STATIC_LATESTBLOCK, height);
    state::store_mapping_u64_bytes32(
        MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
        authority_set_id,
        authority_set_hash,
    );
    state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, authority_set_id);

    // Call executed without any errors, return true.
    true
}

// Only the guardian can update the block range data.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "update_block_range_data")]
#[no_mangle]
pub extern "C" fn update_block_range_data(
    tx_context: *const TxContext,
    ptr: *const u8,
    len: u32,
) -> bool {
    // Decode msg_sender from tx_context and inputs from UpdateBlockRangeDataInput.
    let msg_sender = TxContext::unpack(tx_context).msg_sender();
    let (
        start_blocks,
        end_blocks,
        header_hashes,
        data_root_commitments,
        state_root_commitments,
        end_authority_set_id,
        end_authority_set_hash,
    ) = UpdateBlockRangeDataInput::new(ptr, len).unpack();

    // Fetch the guardian address from the state and check if the msg_sender is the guardian.
    let gaurdian = state::get_address(STATIC_GUARDIAN);
    if msg_sender != gaurdian {
        // msg_sender is not the guardian, return false.
        return false;
    }

    // msg_sender is the guardian.

    // sanity check for the input lengths.
    if !(start_blocks.len() > 0
        && start_blocks.len() == end_blocks.len()
        && end_blocks.len() == header_hashes.len()
        && header_hashes.len() == data_root_commitments.len()
        && data_root_commitments.len() == state_root_commitments.len())
    {
        return false;
    }

    let latest_block = state::get_u32(STATIC_LATESTBLOCK);
    if start_blocks[0] != latest_block {
        // start block is not the latest block.
        return false;
    }

    for i in 0..start_blocks.len() {
        if i < start_blocks.len() - 1 {
            if !(end_blocks[i] == start_blocks[i + 1]) {
                // invalid block range.
                return false;
            }
        }

        // key is the keccak256 hash of the latest block and target block.
        let key = keccak256(
            KeyPacker {
                latestBlock: start_blocks[i],
                targetBlock: end_blocks[i],
            }
            .abi_encode(),
        );

        // store the data and state commitments for the block range.
        state::store_mapping_bytes32_bytes32(
            MAPPING_DATA_ROOT_COMMITMENTS_ID,
            key,
            data_root_commitments[i],
        );
        state::store_mapping_bytes32_bytes32(
            MAPPING_STATE_ROOT_COMMITMENTS_ID,
            key,
            state_root_commitments[i],
        );

        // store the range start blocks.
        state::store_mapping_bytes32_u32(MAPPING_RANGE_START_BLOCKS_ID, key, start_blocks[i]);
        state::store_mapping_u32_bytes32(
            MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID,
            end_blocks[i],
            header_hashes[i],
        );
    }

    // store latest block, end_authority_set_hash, and latest_authority_set_id.
    state::store_u32(STATIC_LATESTBLOCK, end_blocks[end_blocks.len() - 1]);
    state::store_mapping_u64_bytes32(
        MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
        end_authority_set_id,
        end_authority_set_hash,
    );
    state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, end_authority_set_id);

    // Call executed without any errors, return true.
    true
}

/// Add target header hash, and data + state commitments for (latestBlock, targetBlock].
/// The trusted block and requested block must have the same authority set id. If the target
/// block is greater than the max batch size of the circuit, the proof will fail to generate.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "commit_header_range")]
#[no_mangle]
pub extern "C" fn commit_header_range(_: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // unpack proof and public values from CommitHeaderRangeInput.
    let (proof, public_values) = CommitHeaderRangeAndRotateInput::new(ptr, len).unpack();
    let (proof_type, header_range_outputs, _) =
        CommitHeaderRangeAndRotateInput::new(ptr, len).unpack_po();

    // if proof type is not HeaderRangeProof, return false.
    if proof_type != ProofType::HeaderRangeProof {
        return false;
    }

    // if contract is frozen or not initialized, return false.
    if is_frozen() || !is_initialized() {
        return false;
    }

    // unpack the header range outputs.
    let (
        trusted_block,
        trusted_header_hash,
        authority_set_id,
        authority_set_hash,
        target_block,
        target_header_hash,
        state_root_commitment,
        data_root_commitment,
        merkle_tree_size,
    ) = HeaderRangeOutputs::new(&header_range_outputs).unpack();

    // fetch header range merkle tree size from the state.
    let header_range_merkle_tree_size = state::get_u32(STATIC_HEADER_RANGE_COMMITMENT_TREE_SIZE);
    if merkle_tree_size != header_range_merkle_tree_size {
        // invalid merkle tree size.
        return false;
    }

    // fetch the latest block and trusted header hash from the state.
    let latest_block = state::get_u32(STATIC_LATESTBLOCK);
    let stored_trusted_header_hash =
        state::get_mapping_u32_bytes32(MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID, latest_block);

    // sanity check trusted header hash.
    if stored_trusted_header_hash == FixedBytes::new([0; 32]) {
        // trusted header not found.
        return false;
    }

    // check if the trusted header hash and trusted block from state matches the input.
    if trusted_header_hash != stored_trusted_header_hash && trusted_block != latest_block {
        return false;
    }

    // fetch the authority set hash for the authority set id from the state.
    let stored_authority_set_hash =
        state::get_mapping_u64_bytes32(MAPPING_AUTHORITY_SET_ID_TO_HASH_ID, authority_set_id);

    // sanity check authority set hash.
    if stored_authority_set_hash == FixedBytes::<32>::new([0; 32]) {
        return false;
    }

    // check if the authority set hash from the state matches the input.
    if authority_set_hash != stored_authority_set_hash {
        return false;
    }

    // return false if target block is less than or equal to the latest block stored in state.
    if target_block <= latest_block {
        return false;
    }

    // fetch latest authority set id from the state.
    let latest_authority_set_id = state::get_u64(STATIC_LATEST_AUTHORITY_SET_ID);
    if authority_set_id < latest_authority_set_id {
        // old authority set id.
        return false;
    }

    // fetch the program vkey hash and program vkey from the state.
    let (program_vkey_hash, vkey) = get_vkey_hash_and_vkey();
    // verify sp1 plonk proof.
    if precompiles::gnark_verify(program_vkey_hash, public_values, proof, vkey) {
        // proof is valid. update the state variables.
        if authority_set_id > latest_authority_set_id {
            state::store_u64(STATIC_LATEST_AUTHORITY_SET_ID, authority_set_id);
        }
        // key is the keccak256 hash of the latest block and target block.
        let key = keccak256(
            KeyPacker {
                latestBlock: trusted_block,
                targetBlock: target_block,
            }
            .abi_encode(),
        );

        state::store_mapping_bytes32_bytes32(
            MAPPING_DATA_ROOT_COMMITMENTS_ID,
            key,
            data_root_commitment,
        );
        state::store_mapping_bytes32_bytes32(
            MAPPING_STATE_ROOT_COMMITMENTS_ID,
            key,
            state_root_commitment,
        );
        state::store_mapping_bytes32_u32(MAPPING_RANGE_START_BLOCKS_ID, key, latest_block);
        state::store_mapping_u32_bytes32(
            MAPPING_BLOCK_HEIGHT_TO_HEADER_HASH_ID,
            target_block,
            target_header_hash,
        );
        state::store_u32(STATIC_LATESTBLOCK, target_block);

        // Call executed without any errors, return true.
        true
    } else {
        // proof is invalid, return false.
        false
    }
}

/// Adds the authority set hash for the next authority set id.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "rotate")]
#[no_mangle]
pub unsafe extern "C" fn rotate(_: *const TxContext, ptr: *const u8, len: u32) -> bool {
    // unpack proof and public values from CommitHeaderRangeInput.
    let (proof, public_values) = CommitHeaderRangeAndRotateInput::new(ptr, len).unpack();
    let (proof_type, _, rotate_outputs) =
        CommitHeaderRangeAndRotateInput::new(ptr, len).unpack_po();

    // if proof type is not RotateProof, return false.
    if proof_type != ProofType::RotateProof {
        // invalid proof type.
        return false;
    }

    // if contract is frozen or not initialized, return false.
    if is_frozen() || !is_initialized() {
        return false;
    }

    // unpack the rotate outputs.
    let (current_authority_set_id, current_authority_set_hash, new_authority_set_hash) =
        RotateOutputs::new(&rotate_outputs).unpack();

    // fetch the authority set hash for the current authority set id from the state.
    let stored_authority_set_hash = state::get_mapping_u64_bytes32(
        MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
        current_authority_set_id,
    );
    // sanity check authority set hash.
    if stored_authority_set_hash == FixedBytes::<32>::new([0; 32]) {
        return false;
    }
    // check if the authority set hash from the state matches the input.
    if current_authority_set_hash != stored_authority_set_hash {
        return false;
    }

    // fetch the next authority set hash from the state.
    let next_authority_set_hash = state::get_mapping_u64_bytes32(
        MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
        current_authority_set_id + 1,
    );

    // return false if next authority set hash is not empty.
    if next_authority_set_hash != FixedBytes::<32>::new([0; 32]) {
        // next authority set exists
        return false;
    }

    // fetch the program vkey hash and program vkey from the state.
    let (program_vkey_hash, vkey) = get_vkey_hash_and_vkey();
    // verify sp1 plonk proof.
    if precompiles::gnark_verify(program_vkey_hash, public_values, proof, vkey) {
        // proof is valid. update the state variables.

        // store the authority set hash for the next authority set id
        state::store_mapping_u64_bytes32(
            MAPPING_AUTHORITY_SET_ID_TO_HASH_ID,
            current_authority_set_id + 1,
            new_authority_set_hash,
        );

        // Call executed without any errors, return true.
        true
    } else {
        // proof is invalid. return false.
        false
    }
}

// Helper functions

/// Returns true if the contract is frozen, false otherwise.
fn is_frozen() -> bool {
    if state::get_bool(STATIC_FROZEN) == 1 {
        true
    } else {
        false
    }
}

/// Returns true if the contract is initialized, false otherwise.
fn is_initialized() -> bool {
    if state::get_bool(STATIC_ISINITIALIZED) == 1 {
        true
    } else {
        false
    }
}

/// Returns the program vkey hash and program vkey.
fn get_vkey_hash_and_vkey() -> (Vec<u8>, Vec<u8>) {
    let vkey_hash = state::get_vec(STATIC_VECTOR_PROGRAM_VKEY_HASH);
    let vkey = state::get_vec(STATIC_VECTOR_PROGRAM_VKEY);
    (vkey_hash, vkey)
}
