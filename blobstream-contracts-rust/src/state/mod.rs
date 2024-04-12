use crate::{slice, FixedBytes, SolValue, U256};

#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "stateStoreBytes"]
    pub fn store_bytes(slot: u32, ptr: u32, size: u32);
    #[link_name = "stateGetBytes"]
    pub fn get_bytes(slot: u32) -> u64;
    #[link_name = "stateStoreDynamicBytes"]
    pub fn store_dynamic_bytes(offset: u32, key: u32, ptr: u32, size: u32);
    #[link_name = "stateGetDynamicBytes"]
    pub fn get_dynamic_bytes(offset: u32, key: u32) -> u64;
}

pub fn store_u256(variable: u32, value: U256) {
    let value_bytes = value.to_be_bytes_vec();
    unsafe {
        store_bytes(
            variable,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        );
    }
}

pub fn get_u256(variable: u32) -> U256 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        U256::from_be_slice(data)
    }
}

pub fn store_bytes32(variable: u32, value: FixedBytes<32>) {
    let value_bytes = value.abi_encode();
    unsafe {
        store_bytes(
            variable,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        );
    }
}

pub fn get_bytes32(variable: u32) -> FixedBytes<32> {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}

pub fn store_mapping_u256_bytes32(offset: u32, key: U256, value: FixedBytes<32>) {
    if offset == 0 {
        panic!("offset should not be zero");
    }
    let value_bytes = value.abi_encode();
    unsafe {
        let pseudo_key = (key % U256::from(896)).as_limbs()[0]; // offload this to runtime?? @todo
        store_dynamic_bytes(
            offset,
            pseudo_key as u32,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        )
    };
}

pub fn get_mapping_u256_bytes32(offset: u32, key: U256) -> FixedBytes<32> {
    if offset == 0 {
        panic!("offset should not be zero");
    }
    unsafe {
        let pseudo_key = (key % U256::from(896)).as_limbs()[0];
        let ptr_packed = get_dynamic_bytes(offset, pseudo_key as u32);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        FixedBytes::from_slice(data)
    }
}
// use enums for state variables & provide enough abstraction

// Storage layout:
// # storage keys: 1024(default) --> # of storage keys accesible by the contract
// # static keys: 128(default) --> # of storage keys allocated for static varialbes(Uint256, Bytes32...)
// # dynamic keys: 896(default) --> # of storage keys allocated for dynamic types --> mapping, arrays --> after 896 keys are utilised, keys are wrapped to 129 again(clock arithmatic)
// # of storage keys and allocation of storage keys are set during contract deployement. Can be modified by SEQ midway on necessary(ex: when celestia is out)

// for celestia's blobstream case:
// 3 static variables --> state_lastValidatorSetCheckPoint, state_powerThereshold, state_eventNonce
// 1 dynamic variable type --> mapping(uint256 => bytes32) state_dataRootTupleRoots
// this encourages validators to post blocks as early as possible.

// check how frequently celesia posts block roots to ethereum??
// @todo
