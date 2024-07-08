# SEQ-WASM-SDK

SDK for wasm smart contracts on SEQ.

## Usage

- public use allocator module.
```rust pub use seq_wasm_sdk::allocator::*; ```

- for public functions intented to be callable by runtime, function should take 3 inputs, one for `TxContext` pointer and other 2 for a `ptr` and `len`, which can be used for passing inputs to the public function. function should return a `bool` upon completion. `true` equals successful execution and `false` equals unsuccessful execution, any state changes made will be reverted.

```rust 
#[cfg_attr(all(target_arch = "wasm32"), export_name = "initializer")]
#[no_mangle]
pub extern "C" fn function(tx_context: *const TxContext, ptr: *const u8, len: u32) -> bool
```

- every contract should have a `initializer`, which will be called during contract deployment. If the contract does not have need for initializer, use a no-op function as `initializer`.

## Modules

### Allocator:

contains extern memory allocator and deallocater called from go runtime.

| Function name | Inputs |
|-------------| ------------|
| allocate | size: usize |
| deallocate | ptr: *mut u8, size: usize |

### Pre Compiles:

precompiles for wasm smart contracts.

| Precompile | Inputs | Outputs |
|-------------| ------------| ------------|
| ganrk_verify |program_vkey_hash: Vec<u8>, public_values: Vec<u8>,proof: Vec<u8>,program_vkey: Vec<u8> | valid bool | 
| set_balance | address: types::Address, asset: types::ID, amount: u64 | _ |
| get_balance | address: types::Address, asset: types::ID | u64 |


### State:

module contains `externs` for on chain state interactions.

| Extern name | Inputs | Outputs | 
|-------------| ------------| ------------|
| store_bytes | slot: u32, ptr: u32, size: u32 | _ |
| get_bytes | slot: u32 | packed_ptr: u64 |
| store_dynamic_bytes | id: u32, ptr_of_key: u32, size_of_key: u32, ptr: u32, size: u32 | _ |
| get_dynamic_bytes | id: u32, ptr_of_key: u32, size_of_key: u32 | packed_ptr: u64 |

Every contract has 128 static storage slot(s), along with any number of dynamic storage slots. Static storage slots can be used for storing simple types apart from mapping and arrays.`store_bytes` and `get_bytes` store and get values to the static storage slots. Dynamic storage slots are used for storing mappings and arrays. Key for dynamic storage slot also contains `id` unique to a mapping or an array. `store_dynamic_bytes` and `get_dynamic_bytes` store and get values to the dynamic slots.

Abstractions are provided for storing and getting some data types:

| Function Name |
| ------------- |
| store_u256|
| get_u256 |
| store_u64 |
| get_u64 |
| store_u32 |
| get_u32 |
| store_bytes32 | 
| get_bytes32 | 
| store_vec |
| get_vec |
| store_bool |
| get_bool |
| store_mapping_u256_bytes32 |
| get_mapping_u256_bytes32 | 
| store_mapping_u64_bytes32 |
| get_mapping_u64_bytes32 | 
| store_mapping_u32_bytes32|
| get_mapping_u32_bytes32 | 
| store_mapping_bytes32_bytes32 | 
| get_mapping_bytes32_bytes32 |
| store_mapping_bytes32_u32 |
| store_address |
| get_address | 
| store_id | 
| get_id |


### Types:

HyperSDK compatible types.

| Type | Fields |
| ---- | ---- |
| Address | [u8; 33]|
| ID | [u8; 32] |

### Utils:

utility functions and types for Tx. 

| Type | Fields | 
| ---- | ------ |
| TxContext | time_stamp: i64, msg_sender_ptr: u32 |
| gnarkPreCompileInputs | programVKeyHash: Vec<u8>, publicValues: Vec<u8>, proofBytes: Vec<u8>, programVKey: Vec<u8> |

TxContext need to be unpacked, before using. TxContext has some helper function(s).
| Function Name | Inputs | Outputs | 
| ------------- | ------ | ------- |
| unpack | ptr: *const TxContext | Self | 
| msg_sender | _ | types::Address |
| time_stamp | _ | i64 | 