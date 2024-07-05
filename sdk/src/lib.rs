#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
pub extern crate alloc;
pub extern crate core;
pub extern crate wee_alloc;

pub mod allocator;
pub mod state;
pub mod utils;
// alloy re-exports
pub use alloy_primitives::{
    bytes, fixed_bytes, hex::FromHex, keccak256, Address, Bytes, FixedBytes, Uint, B256, U256, U64,
};
pub use alloy_sol_macro::sol;
pub use alloy_sol_types::{SolType, SolValue};

// std lib re-exports
pub use core::slice;
pub use std::alloc::{alloc, Layout};
pub use std::mem::MaybeUninit;
