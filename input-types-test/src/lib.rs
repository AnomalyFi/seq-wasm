extern crate alloc;
extern crate core;
extern crate wee_alloc;
use core::slice;
pub use seq_wasm_sdk::allocator::{allocate, deallocate}; // re-export
use seq_wasm_sdk::state::get_bytes;
pub use std::alloc::{alloc, Layout};

#[no_mangle]
pub extern "C" fn test_get_bytes() -> u64 {
    unsafe {
        let ptr_packed = get_bytes(3);
        let sdrtr_input =
            slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());

        if sdrtr_input == vec![0 as u8, 2, 3] {
            return 10;
        }
        return 0;
    };
}
