# SEQ-WASM Framework

Branch X is the implementation of [BlobstreamX](https://github.com/succinctlabs/blobstreamx/tree/main/contracts/src/BlobstreamX.sol)

- Implementation of BlobstreamX over SEQ involves adding a syscall/precompile to wasm runtime to verify the gnark proof.

- Block header transition check(trusted block -> new block) should be satisfied and implemented properly, so arbitary proofs are not submitted.

- Fall backs for fund dispersion needs to be implemented.

- Necessary design decisions will be either explained here or in the relayer(to be built[this realys the proof automatically]) or in add-wasm branch of nodekit-seq.

TODO:
- [x] Remove redundent code.
- [ ] Raw blobstream x translation with modifications to contain gateway functionality in itself.
    - We essentially need commitNextHeader, commitHeaderRange and verify attestation.
    - Change intializer implementation.
    - implement lock functionality.
    - state variables change.
- [ ] Add precompile in wasm runtime. 
- [ ] Add tests. 
- [ ] Fallback mechanisms acting as post-execution hooks in wasm runtime. Ex: disburse fee after valid attesatation, call status fallback -> success or failure. may be something similar to events.

- [ ] Possibly state revamp into more dyanmic generations.
- [ ] Use macros for public functions and state variables.
- [ ] Switch to JSON types.??
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