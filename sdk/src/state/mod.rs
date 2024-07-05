use crate::utils::gnarkPrecompileInputs;
use crate::{slice, Address, FixedBytes, SolValue, U256};
#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "stateStoreBytes"]
    pub fn store_bytes(slot: u32, ptr: u32, size: u32);
    #[link_name = "stateGetBytes"]
    pub fn get_bytes(slot: u32) -> u64;
    #[link_name = "stateStoreDynamicBytes"]
    pub fn store_dynamic_bytes(id: u32, ptr_of_key: u32, size_of_key: u32, ptr: u32, size: u32);
    #[link_name = "stateGetDynamicBytes"]
    pub fn get_dynamic_bytes(id: u32, ptr_of_key: u32, size_of_key: u32) -> u64;
    /// SP1 plonk verify precompile.
    /// Returns 1 for valid proof, 0 otherwise.
    /// Verifies the proof and public values without any checks for invarients.
    /// Invarient checks should be performed before calling the precompile..
    /// ptr & size are of the struct `gnarkPrecompileInputs`
    #[link_name = "gnarkVerify"]
    pub fn gnark_verify_inner(ptr: u32, size: u32) -> u32;
}

pub unsafe fn gnark_verify(
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

    let valid = gnark_verify_inner(data.as_ptr() as u32, data.len() as u32);
    if valid == 1 {
        true
    } else {
        false
    }
}

pub fn store_u256(variable: u32, value: U256) {
    let value_bytes = value.to_be_bytes_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_u256(variable: u32) -> U256 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        U256::from_be_slice(data)
    }
}

pub fn store_u64(variable: u32, value: u64) {
    let value_bytes = value.to_be_bytes();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_u64(variable: u32) -> u64 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u64::from_be_bytes(data.try_into().unwrap())
    }
}

pub fn store_u32(variable: u32, value: u32) {
    let value_bytes = value.to_be_bytes();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_u32(variable: u32) -> u32 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u32::from_be_bytes(data.try_into().unwrap())
    }
}

pub fn store_bytes32(variable: u32, value: FixedBytes<32>) {
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_bytes32(variable: u32) -> FixedBytes<32> {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_address(variable: u32, value: Address) {
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_address(variable: u32) -> Address {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        Address::from_slice(data)
    }
}

pub fn store_vec(variable: u32, vector: &[u8]) {
    let vector = vector.to_vec();
    let ptr = vector.as_ptr() as u32;
    let len = vector.len() as u32;
    std::mem::forget(vector);
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_vec(variable: u32) -> Vec<u8> {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        data.to_vec()
    }
}

pub fn store_bool(variable: u32, value: u32) {
    let value_bytes = if value == 0 {
        0_u32.to_be_bytes()
    } else {
        1_u32.to_be_bytes()
    };
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    unsafe {
        store_bytes(variable, ptr, len);
    }
}

pub fn get_bool(variable: u32) -> u32 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        if u32::from_be_bytes(data.try_into().unwrap()) == 0 {
            0
        } else {
            1
        }
    }
}

pub fn store_mapping_u256_bytes32(id: u32, key: U256, value: FixedBytes<32>) {
    // get ptr and len of key. forget the key.
    let key = key.to_be_bytes_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    // get ptr and len of value. forget the value.
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);

    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

pub fn get_mapping_u256_bytes32(id: u32, key: U256) -> FixedBytes<32> {
    // get ptr and len of key. forget the key.
    let key = key.to_be_bytes_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_mapping_u64_bytes32(id: u32, key: u64, value: FixedBytes<32>) {
    let key = key.to_be_bytes();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;

    // get ptr and len of value. forget the value.
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);

    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

pub fn get_mapping_u64_bytes32(id: u32, key: u64) -> FixedBytes<32> {
    let key = key.to_be_bytes();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_mapping_u32_bytes32(id: u32, key: u32, value: FixedBytes<32>) {
    let key = key.to_be_bytes();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;

    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

pub fn get_mapping_u32_bytes32(id: u32, key: u32) -> FixedBytes<32> {
    let key = key.to_be_bytes();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_mapping_bytes32_bytes32(id: u32, key: FixedBytes<32>, value: FixedBytes<32>) {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

pub fn get_mapping_bytes32_bytes32(id: u32, key: FixedBytes<32>) -> FixedBytes<32> {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_mapping_bytes32_u32(id: u32, key: FixedBytes<32>, value: u32) {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    let value_bytes = value.to_be_bytes();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    // std::mem::forget(value_bytes);
    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

pub fn get_mapping_bytes32_u32(id: u32, key: FixedBytes<32>) -> u32 {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u32::from_be_bytes(data.try_into().unwrap())
    }
}
