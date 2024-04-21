use std::slice;

#[repr(C)]
pub struct TxContext {
    pub time_stamp: i64,
    pub msg_sender_ptr: u32,
    pub msg_sender_len: u32,
}

impl TxContext {
    pub fn msg_sender<'a>(&self) -> Vec<u8> {
        unsafe { ptr_to_address_bytes(self.msg_sender_ptr, self.msg_sender_len) }
    }
}

pub unsafe fn ptr_to_address_bytes<'a>(ptr: u32, len: u32) -> Vec<u8> {
    slice::from_raw_parts(ptr as *mut u8, len as usize).to_vec()
}
