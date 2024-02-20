# SEQ-WASM Framework

Stateful Wasm contract support for SEQ

Every rust contract should implement this:
```rust 
extern crate alloc;
extern crate core;
extern crate wee_alloc;

use std::alloc::{alloc, Layout};
use std::mem::MaybeUninit;
use alloc::vec::Vec;
use crate::tx::Info;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Allocates size bytes and leaks the pointer where they start.
#[cfg_attr(all(target_arch = "wasm32"), export_name = "allocate_ptr")]
#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    // Allocate the amount of bytes needed.
    let vec: Vec<MaybeUninit<u8>> = Vec::with_capacity(size);

    // into_raw leaks the memory to the caller.
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

#[cfg_attr(all(target_arch = "wasm32"), export_name = "deallocate_ptr")]
#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);
}

```

outputs should be `u64`: 1 -> success. Anything else is a failure.