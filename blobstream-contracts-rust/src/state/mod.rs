use crate::{slice, FixedBytes, SolValue, HEADER_RANGE_FUNCTION_ID, U256};

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
    /// Groth16 verify precompile
    /// Returns 1 if the proof is valid, 0 otherwise
    /// trusted_block is input. The input of the caller function will be decoded to find input, output, proof
    #[link_name = "gnarkVerify"]
    pub fn gnark_verify_inner(trusted_block: u64, ptr: u32, size: u32) -> u32;
}

pub unsafe fn gnark_verify(trusted_block: u64) -> bool {
    let valid = gnark_verify_inner(
        trusted_block,
        HEADER_RANGE_FUNCTION_ID.as_ptr() as u32,
        HEADER_RANGE_FUNCTION_ID.len() as u32,
    );
    if valid == 1 {
        true
    } else {
        false
    }
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

pub fn store_u64(variable: u32, value: u64) {
    let value_bytes = value.to_be_bytes();
    unsafe {
        store_bytes(
            variable,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        );
    }
}

pub fn get_u64(variable: u32) -> u64 {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        u64::from_be_bytes(data.try_into().unwrap())
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

pub fn store_vec(variable: u32, data: &[u8]) {
    unsafe {
        store_bytes(variable, data.as_ptr() as u32, data.len() as u32);
    }
}

pub fn get_vec(variable: u32) -> Vec<u8> {
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        data.to_vec()
    }
}
pub fn store_bool(variable: u32, value: u32) {
    let value_bytes = if value == 0 {
        0_u32.to_be_bytes()
    } else {
        1_u32.to_be_bytes()
    };
    unsafe {
        store_bytes(
            variable,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        );
    }
}

pub fn get_bool(variable: u32) -> u32 {
    //@todo revamp
    unsafe {
        let ptr_packed = get_bytes(variable);
        let data = slice::from_raw_parts((ptr_packed >> 32) as *mut u8, (ptr_packed as u16).into());
        if u32::from_be_bytes(data.try_into().unwrap()) == 0 {
            0
        } else {
            1
        }
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

pub fn store_mapping_u64_bytes32(offset: u32, key: u64, value: FixedBytes<32>) {
    if offset == 0 {
        panic!("offset should not be zero");
    }
    let value_bytes = value.abi_encode();
    unsafe {
        let pseudo_key = key % 896; // offload this to runtime?? @todo
        store_dynamic_bytes(
            offset,
            pseudo_key as u32,
            value_bytes.as_ptr() as u32,
            value_bytes.len() as u32,
        )
    };
}

pub fn get_mapping_u64_bytes32(offset: u32, key: u64) -> FixedBytes<32> {
    if offset == 0 {
        panic!("offset should not be zero");
    }
    unsafe {
        let pseudo_key = key % 896;
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

// check how frequently celestia posts block roots to ethereum?? -> every 300 blocks.
// @todo
