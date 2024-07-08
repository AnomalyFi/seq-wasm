use crate::{slice, sol, types};

/// Contains the Context of the Transaction. This is passed to the contract as a pointer from go runtime.
#[repr(C)]
pub struct TxContext {
    // time_stamp of block, the transaction is included in.
    pub time_stamp: i64,
    // ptr to msg_sender(caller) of the transaction.
    pub msg_sender_ptr: u32,
}

impl TxContext {
    /// Unpacks the TxContext from the pointer.
    pub fn unpack(ptr: *const TxContext) -> Self {
        unsafe { ptr.read() }
    }

    /// Returns the msg_sender(caller) of the transaction.
    pub fn msg_sender(&self) -> types::Address {
        let msg_sender_bytes =
            unsafe { slice::from_raw_parts(self.msg_sender_ptr as *mut u8, types::Address::LEN) };
        types::Address::new(msg_sender_bytes.try_into().unwrap())
    }

    /// Returns the time_stamp of the block, the transaction is included in.
    pub fn time_stamp(&self) -> i64 {
        self.time_stamp
    }
}

sol!(
    /// Input for gnark precompile.
    struct gnarkPrecompileInputs{
        bytes programVKeyHash;
        bytes publicValues;
        bytes proofBytes;
        bytes programVKey;
    }
);
