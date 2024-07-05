use crate::types;
use crate::utils::gnarkPrecompileInputs;
use crate::{FixedBytes, SolValue};

#[link(wasm_import_module = "precompiles")]
extern "C" {
    /// SP1 plonk verify precompile.
    /// Returns 1 for valid proof, 0 otherwise.
    /// Verifies the proof and public values without any checks for invarients.
    /// Invarient checks should be performed before calling the precompile..
    /// ptr & size are of the struct `gnarkPrecompileInputs`
    #[link_name = "gnarkVerify"]
    pub fn gnark_verify_inner(ptr: u32, size: u32) -> u32;

    /// Set balance of an address for an asset.
    #[link_name = "setBalance"]
    pub fn set_balance_inner(address_ptr: u32, asset_ptr: u32, amount: u64);
    /// Get balance of an address for an asset.
    #[link_name = "getBalance"]
    pub fn get_balance_inner(address_ptr: u32, asset_ptr: u32) -> u64;
}

pub fn gnark_verify(
    program_vkey_hash: FixedBytes<32>,
    public_values: Vec<u8>,
    proof: Vec<u8>,
    program_vkey: Vec<u8>,
) -> bool {
    let data = gnarkPrecompileInputs {
        programVKeyHash: program_vkey_hash,
        publicValues: public_values,
        proofBytes: proof,
        programVKey: program_vkey,
    }
    .abi_encode();

    let valid = unsafe { gnark_verify_inner(data.as_ptr() as u32, data.len() as u32) };
    if valid == 1 {
        true
    } else {
        false
    }
}

pub fn set_balance(address: types::Address, asset: types::ID, amount: u64) {
    let address = address.as_bytes().to_vec();
    let addr_ptr = address.as_ptr() as u32;
    std::mem::forget(address);
    let asset = asset.as_bytes().to_vec();
    let asset_ptr = asset.as_ptr() as u32;
    std::mem::forget(asset);
    unsafe { set_balance_inner(addr_ptr, asset_ptr, amount) };
}

pub fn get_balance(address: types::Address, asset: types::ID) -> u64 {
    let address = address.as_bytes().to_vec();
    let addr_ptr = address.as_ptr() as u32;
    std::mem::forget(address);
    let asset = asset.as_bytes().to_vec();
    let asset_ptr = asset.as_ptr() as u32;
    std::mem::forget(asset);
    unsafe { get_balance_inner(addr_ptr, asset_ptr) }
}
