extern crate alloc;
extern crate core;
extern crate wee_alloc;

pub use seq_wasm_sdk::allocator::{allocate, deallocate}; // re-export
use seq_wasm_sdk::state::{self};
use seq_wasm_sdk::{precompiles, FixedBytes, U256};
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
pub extern "C" fn test_store_mapping_u256_bytes32() {
    state::store_mapping_u256_bytes32(1, U256::from(569), FixedBytes::from([1; 32]));
}

#[no_mangle]
pub extern "C" fn test_get_mapping_u256_bytes32() -> bool {
    state::store_mapping_u256_bytes32(2, U256::from(569), FixedBytes::from([1; 32]));
    let bytes32_fs = state::get_mapping_u256_bytes32(2, U256::from(569));
    if bytes32_fs == [1; 32] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_mapping_u64_bytes32() {
    state::store_mapping_u64_bytes32(3, 4230, FixedBytes::from([1; 32]));
}

#[no_mangle]
pub extern "C" fn test_get_mapping_u64_bytes32() -> bool {
    state::store_mapping_u64_bytes32(4, 4230, FixedBytes::from([1; 32]));
    let bytes32_fs = state::get_mapping_u64_bytes32(4, 4230);
    if bytes32_fs == [1; 32] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_mapping_u32_bytes32() {
    state::store_mapping_u32_bytes32(5, 123, FixedBytes::from([1; 32]));
}

#[no_mangle]
pub extern "C" fn test_get_mapping_u32_bytes32() -> bool {
    state::store_mapping_u32_bytes32(6, 123, FixedBytes::from([1; 32]));
    let bytes32_fs = state::get_mapping_u32_bytes32(6, 123);
    if bytes32_fs == [1; 32] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_mapping_bytes32_bytes32() {
    state::store_mapping_bytes32_bytes32(7, FixedBytes::from([2; 32]), FixedBytes::from([4; 32]));
}

#[no_mangle]
pub extern "C" fn test_get_mapping_bytes32_bytes32() -> bool {
    state::store_mapping_bytes32_bytes32(8, FixedBytes::from([2; 32]), FixedBytes::from([4; 32]));
    let bytes32_fs = state::get_mapping_bytes32_bytes32(8, FixedBytes::from([2; 32]));
    if bytes32_fs == [4; 32] {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_store_mapping_bytes32_u32() {
    state::store_mapping_bytes32_u32(9, FixedBytes::from([2; 32]), 12369);
}

#[no_mangle]
pub extern "C" fn test_get_mapping_bytes32_u32() -> bool {
    state::store_mapping_bytes32_u32(10, FixedBytes::from([2; 32]), 12369);
    let u32_fs = state::get_mapping_bytes32_u32(10, FixedBytes::from([2; 32]));
    if u32_fs == 12369 {
        return true;
    }
    false
}

#[no_mangle]
pub extern "C" fn test_precompiles_module_call() -> bool {
    let fb = FixedBytes::from([1; 32]);
    precompiles::gnark_verify(fb, vec![1u8], vec![1u8], vec![1u8])
}

#[no_mangle]
pub extern "C" fn test_multi_input(a: u32, b: u64, c: u32, d: u32) -> u32 {
    a + b as u32 + c + d
}
