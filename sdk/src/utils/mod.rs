use crate::{sol, types};
use std::slice;

#[repr(C)]
pub struct TxContext {
    pub time_stamp: i64,
    pub msg_sender_ptr: u32,
}

impl TxContext {
    pub fn msg_sender(&self) -> types::Address {
        let msg_sender_bytes =
            unsafe { slice::from_raw_parts(self.msg_sender_ptr as *mut u8, types::Address::LEN) };
        types::Address::new(msg_sender_bytes.try_into().unwrap())
    }
}

sol!(
    struct gnarkPrecompileInputs{
        bytes32 programVKeyHash;
        bytes publicValues;
        bytes proofBytes;
        bytes programVKey;
    }
);
