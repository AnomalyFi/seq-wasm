// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
extern crate alloc;
extern crate core;
extern crate wee_alloc;

// std lib
// use core::slice;
pub use std::alloc::{alloc, Layout};
use std::mem::MaybeUninit;

// seq wasm sdk
use seq_wasm_sdk::allocator::*;
use seq_wasm_sdk::state;
use seq_wasm_sdk::utils;

const STATIC_ISINITIALIZED: u32 = 0;
const STATIC_FROZEN: u32 = 1;
const STATIC_OWNER: u32 = 2;
const STATIC_LATESTBLOCK: u32 = 3;
const STATIC_LATEST_AUTHORITY_SET_ID: u32 = 4;
const STATIC_HEADER_RANGE_FUNCTION_ID: u32 = 5;
const STATIC_ROTATE_FUNCTION_ID: u32 = 6;

const DYNAMIC_BLOCK_HEIGHT_TO_HEADER_HASH: u32 = 2;
const DYNAMIC_AUTHORITY_SET_ID_TO_HASH: u32 = 3;
const DYAMIC_DATA_ROOT_COMMITMENTS: u32 = 4;
const DYNAMIC_STATE_ROOT_COMMITMENTS: u32 = 5;
const DYNAMIC_RANGE_START_BLOCKS: u32 = 6;
