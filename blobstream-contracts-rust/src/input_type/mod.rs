use crate::{slice, sol, FixedBytes, SolType, U256};
use crate::{BinaryMerkleProof, DataRootTuple};

sol!(
    struct VAInput {
        uint256 tuple_root_nonce;
        DataRootTuple tuple;
        BinaryMerkleProof proof;
    }
    struct InitializerInput {
        uint256 nonce;
        uint256 power_threshold;
        bytes32 validator_set_check_point;
    }
);

impl VAInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let va_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(va_input, true).unwrap()
    }
    pub fn unpack(&self) -> (U256, DataRootTuple, BinaryMerkleProof) {
        (
            self.tuple_root_nonce,
            self.tuple.clone(),
            self.proof.clone(),
        )
    }
}
impl InitializerInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (U256, U256, FixedBytes<32>) {
        (
            self.nonce,
            self.power_threshold,
            self.validator_set_check_point,
        )
    }
}

//@todo create input handlers for commit header range & commit next header
