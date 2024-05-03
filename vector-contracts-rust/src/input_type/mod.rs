use crate::{slice, sol, FixedBytes, SolType};

sol!(
    struct InitializerInput{
        uint32 height;
        bytes32 header;
        uint64 authoritySetId;
        bytes32 authoritySetHash;
        bytes32 headerRangeFunctionId;
        bytes32 rotateFunctionId;
    }
    struct UpdateFreezeInput{
        bool freeze;
    }
    struct CommitHeaderRangeInput{
        uint64 authoritySetId;
        uint32 targetBlock;
        bytes input;
        bytes output;
        bytes proof;
    }
    struct InputHashPacker {
        uint32 latestBlock;
        bytes32 trustedHeader;
        uint64 authoritySetId;
        bytes32 authoritySetHash;
        uint32 targetBlock;
    }
    struct OutputBreaker{
        bytes32 targetHeaderHash;
        bytes32 stateRootCommitment;
        bytes32 dataRootCommitment;
    }
);
impl InitializerInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(
        &self,
    ) -> (
        u32,
        FixedBytes<32>,
        u64,
        FixedBytes<32>,
        FixedBytes<32>,
        FixedBytes<32>,
    ) {
        (
            self.height,
            self.header,
            self.authoritySetId,
            self.authoritySetHash,
            self.headerRangeFunctionId,
            self.rotateFunctionId,
        )
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
impl CommitHeaderRangeInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (u64, u32, Vec<u8>, Vec<u8>, Vec<u8>) {
        (
            self.authoritySetId,
            self.targetBlock,
            self.input.clone(),
            self.output.clone(),
            self.proof.clone(),
        )
    }
}

impl OutputBreaker {
    pub fn decode(data: &[u8]) -> (FixedBytes<32>, FixedBytes<32>, FixedBytes<32>) {
        let ob = Self::abi_decode(data, true).unwrap();
        (
            ob.targetHeaderHash,
            ob.stateRootCommitment,
            ob.dataRootCommitment,
        )
    }
}
