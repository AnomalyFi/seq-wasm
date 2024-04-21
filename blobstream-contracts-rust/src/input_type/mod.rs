use crate::{slice, sol, FixedBytes, SolType, U256};
use crate::{BinaryMerkleProof, DataRootTuple};

sol!(
    struct VAInput {
        uint256 tuple_root_nonce;
        DataRootTuple tuple;
        BinaryMerkleProof proof;
    }
    struct InitializerInput {
        uint64  height;
        bytes32 header;
    }
    struct CommitHeaderRangeInput {
        uint64 targetBlock;
        bytes input;
        bytes output;
        bytes proof;
    }
    struct UpdateFreezeInput{
        bool freeze;
    }
    struct OutputBreaker{
        bytes32 targetHeader;
        bytes32 dataCommitment;
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
    pub fn unpack(&self) -> (u64, FixedBytes<32>) {
        (self.height, self.header.clone())
    }
}

impl CommitHeaderRangeInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (u64, Vec<u8>, Vec<u8>, Vec<u8>) {
        (
            self.targetBlock,
            self.input.clone(),
            self.output.clone(),
            self.proof.clone(),
        )
    }
}

impl OutputBreaker {
    pub fn decode(data: &[u8]) -> (FixedBytes<32>, FixedBytes<32>) {
        let ob = Self::abi_decode(data, true).unwrap();
        (ob.targetHeader, ob.dataCommitment)
    }
}

impl UpdateFreezeInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> bool {
        self.freeze
    }
}
//@todo create input handlers for commit header range & commit next header
