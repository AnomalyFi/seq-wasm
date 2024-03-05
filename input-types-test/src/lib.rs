extern crate alloc;
extern crate core;
extern crate wee_alloc;
use alloy_primitives::{address, fixed_bytes, U256};
pub use blobstream_contracts_rust::{allocate, deallocate}; // re-export
use blobstream_contracts_rust::{input_type::SDRTRInput, state::get_bytes, Validator};
use core::slice;
pub use std::alloc::{alloc, Layout};

#[no_mangle]
pub extern "C" fn test_sdrtr_input(ptr: *const u8, len: u32) -> u64 {
    let (new_nonce, validator_set_nonce, data_root_tuple_root, current_validators, signatures) =
        SDRTRInput::new(ptr, len).unpack();
    if new_nonce != U256::from(30)
        || validator_set_nonce != U256::from(489039)
        || data_root_tuple_root
            != fixed_bytes!("82dc1607d84557d3579ce602a45f5872e821c36dbda7ec926dfa17ebc8d5c013")
    {
        return 0;
    }
    let val = get_dummy_validators_one();
    if current_validators.is_empty() || signatures.is_empty() {
        return 345;
    }
    if val[0].addr != current_validators[0].addr || val[0].power != current_validators[0].power {
        return 243;
    }
    let v = 27;
    let r = fixed_bytes!("02bd9e5fe41ca09e69c688eb127ba3a710ba0f9f9080b13c1f003126a74be2d5");
    let s = fixed_bytes!("6dc6943fc93d17984e3ac3023b15030b33a5c9b6e647ddfb3a7f19a1c3ce9a2e");

    if signatures[0].v != v || signatures[0].r != r || signatures[0].s != s {
        return 345;
    }
    1208
}

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

fn get_dummy_validators_one() -> Vec<Validator> {
    let pub_addr_1 = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
    let power_1 = U256::from(5000);
    vec![Validator {
        addr: pub_addr_1,
        power: power_1,
    }]
}
