extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub use seq_wasm_sdk::allocator::{allocate, deallocate}; // re-export
use seq_wasm_sdk::state::{self};
use seq_wasm_sdk::{FixedBytes, U256};
pub use std::alloc::{alloc, Layout};

#[no_mangle]
pub extern "C" fn test_store_u256() {
    state::store_u256(1, U256::from(123));
}

#[no_mangle]
pub extern "C" fn test_get_u256() -> bool {
    state::store_u256(2, U256::from(123));
    let u256_fs = state::get_u256(2);
    if u256_fs == U256::from(123) {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_u64() {
    state::store_u64(3, 123);
}

#[no_mangle]
pub extern "C" fn test_get_u64() -> bool {
    state::store_u64(4, 123);
    let u64_fs = state::get_u64(4);
    if u64_fs == 123 {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_u32() {
    state::store_u32(5, 123);
}

#[no_mangle]
pub extern "C" fn test_get_u32() -> bool {
    state::store_u32(6, 123);
    let u32_fs = state::get_u32(6);
    if u32_fs == 123 {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_bool() {
    state::store_bool(7, 1);
}

#[no_mangle]
pub extern "C" fn test_get_bool() -> bool {
    state::store_bool(8, 1);
    let bool_fs = state::get_bool(8);
    if bool_fs == 1 {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_bytes32() {
    state::store_bytes32(9, FixedBytes::from([1; 32]));
}

#[no_mangle]
pub extern "C" fn test_get_bytes32() -> bool {
    state::store_bytes32(10, FixedBytes::from([1; 32]));
    let bytes32_fs = state::get_bytes32(10);
    if bytes32_fs == [1; 32] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_bytes() {
    state::store_vec(11, &vec![1, 2, 3]);
}

#[no_mangle]
pub extern "C" fn test_get_bytes() -> bool {
    state::store_vec(12, &vec![1, 2, 3]);
    let bytes_fs = state::get_vec(12);
    if bytes_fs == vec![1, 2, 3] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_dynamic() {
    let vet = vec![1 as u8, 2, 3];
    let ptr = vet.as_ptr() as u32;
    let size = vet.len() as u32;
    std::mem::forget(vet);
    unsafe { state::store_dynamic_bytes(2, 4, ptr, size) };
}
