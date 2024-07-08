use crate::types;
use crate::{slice, FixedBytes, SolValue, U256};

// Extern linked to wasm module as `env`. The functions are implemented in the go runtime for wasm.
#[link(wasm_import_module = "env")]
extern "C" {
    /// Store bytes in the state at `slot`.
    /// Takes a pointer to the bytes and the size of the bytes to be stored in state.
    /// Any ptr passed should be forgotten with `std::mem::forget()` before calling this function.`
    #[link_name = "stateStoreBytes"]
    pub fn store_bytes(slot: u32, ptr: u32, size: u32);

    /// Get bytes from the state at `slot`.
    /// Returns a u64 where the upper 32 bits are the pointer to the bytes and the lower 32 bits are the size of the bytes.
    #[link_name = "stateGetBytes"]
    pub fn get_bytes(slot: u32) -> u64;

    /// Store bytes in the state at `id` and `key`.
    /// Takes a pointer to the key and the size of the key, a pointer to the bytes and the size of the bytes to be stored.
    #[link_name = "stateStoreDynamicBytes"]
    pub fn store_dynamic_bytes(id: u32, ptr_of_key: u32, size_of_key: u32, ptr: u32, size: u32);

    /// Get bytes from the state at `id` and `key`.
    /// Returns a u64 where the upper 32 bits are the pointer to the bytes and the lower 32 bits are the size of the bytes.
    #[link_name = "stateGetDynamicBytes"]
    pub fn get_dynamic_bytes(id: u32, ptr_of_key: u32, size_of_key: u32) -> u64;
}

/// Stores a u256 value in the state at `key`.
pub fn store_u256(key: u32, value: U256) {
    let value_bytes = value.to_be_bytes_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns u256 from state at `key`. If the key is not found, or the value is not a u256, returns an error.
pub fn get_u256(key: u32) -> U256 {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        U256::from_be_slice(data.try_into().unwrap_or_default())
    }
}

/// Stores a u64 value in the state at `key`.
pub fn store_u64(key: u32, value: u64) {
    let value_bytes = value.to_be_bytes().to_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns u64 from state at `key`. If the key is not found, or the value is not a u64, returns 0.
pub fn get_u64(key: u32) -> u64 {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u64::from_be_bytes(data.try_into().unwrap_or_default())
    }
}

/// Stores a u32 value in the state at `key`.
pub fn store_u32(key: u32, value: u32) {
    let value_bytes = value.to_be_bytes().to_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns u32 from state at `key`. If the key is not found, or the value is not a u32, returns 0.
pub fn get_u32(key: u32) -> u32 {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u32::from_be_bytes(data.try_into().unwrap_or_default())
    }
}

/// Stores a u16 value in the state at `key`.
pub fn store_bytes32(key: u32, value: FixedBytes<32>) {
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns u16 from state at `key`. If the key is not found, or the value is not a u16, returns 0.
pub fn get_bytes32(key: u32) -> FixedBytes<32> {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

/// Stores a [u8] in the state at `key`.
pub fn store_vec(key: u32, vector: &[u8]) {
    let vector = vector.to_vec();
    let ptr = vector.as_ptr() as u32;
    let len = vector.len() as u32;
    std::mem::forget(vector);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns [u8] from state at `key`.
pub fn get_vec(key: u32) -> Vec<u8> {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        data.to_vec()
    }
}

/// Stores a bool value in the state at `key`. If the value is 0, stores 0u32, otherwise stores 1u32.
pub fn store_bool(key: u32, value: u32) {
    let value_bytes = if value == 0 {
        0_u32.to_be_bytes().to_vec()
    } else {
        1_u32.to_be_bytes().to_vec()
    };
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns bool from state at `key`. If the value is 0u32, returns 0, otherwise returns 1.
pub fn get_bool(key: u32) -> u32 {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        if u32::from_be_bytes(data.try_into().unwrap_or_default()) == 0 {
            0
        } else {
            1
        }
    }
}

/// Stores FixedBytes<32> in the state at U256 `key` associated with mapping id `id`.
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

/// Returns FixedBytes<32> from state at U256 `key` associated with mapping id `id`. If the key is not found, returns an empty FixedBytes<32>.
pub fn get_mapping_u256_bytes32(id: u32, key: U256) -> FixedBytes<32> {
    // get ptr and len of key. forget the key.
    let key = key.to_be_bytes_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        match FixedBytes::try_from(data) {
            Ok(fixed_bytes) => fixed_bytes,
            Err(_) => FixedBytes::default(),
        }
    }
}

/// Stores FixedBytes<32> in the state at u64 `key` associated with mapping id `id`.
pub fn store_mapping_u64_bytes32(id: u32, key: u64, value: FixedBytes<32>) {
    let key = key.to_be_bytes().to_vec();
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

/// Returns FixedBytes<32> from state at u64 `key` associated with mapping id `id`. If the key is not found, returns an empty FixedBytes<32>.
pub fn get_mapping_u64_bytes32(id: u32, key: u64) -> FixedBytes<32> {
    let key = key.to_be_bytes().to_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);
    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        match FixedBytes::try_from(data) {
            Ok(fixed_bytes) => fixed_bytes,
            Err(_) => FixedBytes::default(),
        }
    }
}

/// Stores FixedBytes<32> in the state at u32 `key` associated with mapping id `id`.
pub fn store_mapping_u32_bytes32(id: u32, key: u32, value: FixedBytes<32>) {
    let key = key.to_be_bytes().to_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);
    let value_bytes = value.abi_encode();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

/// Returns FixedBytes<32> from state at u32 `key` associated with mapping id `id`. If the key is not found, returns an empty FixedBytes<32>.
pub fn get_mapping_u32_bytes32(id: u32, key: u32) -> FixedBytes<32> {
    let key = key.to_be_bytes().to_vec();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        match FixedBytes::try_from(data) {
            Ok(fixed_bytes) => fixed_bytes,
            Err(_) => FixedBytes::default(),
        }
    }
}

/// Stores FixedBytes<32> in the state at FixedBytes<32> `key` associated with mapping id `id`.
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

/// Returns FixedBytes<32> from state at FixedBytes<32> `key` associated with mapping id `id`. If the key is not found, returns an empty FixedBytes<32>.
pub fn get_mapping_bytes32_bytes32(id: u32, key: FixedBytes<32>) -> FixedBytes<32> {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        match FixedBytes::try_from(data) {
            Ok(fixed_bytes) => fixed_bytes,
            Err(_) => FixedBytes::default(),
        }
    }
}

/// Stores u32 in the state at FixedBytes<32> `key` associated with mapping id `id`.
pub fn store_mapping_bytes32_u32(id: u32, key: FixedBytes<32>, value: u32) {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    let value_bytes = value.to_be_bytes().to_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe { store_dynamic_bytes(id, ptr_key, len_key, ptr, len) };
}

/// Returns u32 from state at FixedBytes<32> `key` associated with mapping id `id`. If the key is not found, returns 0.
pub fn get_mapping_bytes32_u32(id: u32, key: FixedBytes<32>) -> u32 {
    let key = key.abi_encode();
    let ptr_key = key.as_ptr() as u32;
    let len_key = key.len() as u32;
    std::mem::forget(key);

    unsafe {
        let ptr_packed = get_dynamic_bytes(id, ptr_key, len_key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u32::from_be_bytes(data.try_into().unwrap_or_default())
    }
}

/// Stores `Address` in the state at `key`.
pub fn store_address(key: u32, value: &types::Address) {
    let value_bytes = value.as_bytes().to_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns `Address` from state at `key`. If the key is not found, or the value is not an `Address`, returns an empty `Address`.
pub fn get_address(key: u32) -> types::Address {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        types::Address::new(data.try_into().unwrap_or([0; 33]))
    }
}

/// Stores `ID` in the state at `key`.
pub fn store_id(key: u32, value: &types::ID) {
    let value_bytes = value.as_bytes().to_vec();
    let ptr = value_bytes.as_ptr() as u32;
    let len = value_bytes.len() as u32;
    std::mem::forget(value_bytes);
    unsafe {
        store_bytes(key, ptr, len);
    }
}

/// Returns `ID` from state at `key`. If the key is not found, or the value is not an `ID`, returns an empty `ID`.
pub fn get_id(key: u32) -> types::ID {
    unsafe {
        let ptr_packed = get_bytes(key);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        types::ID::new(data.try_into().unwrap_or([0; 32]))
    }
}
