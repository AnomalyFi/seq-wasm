use crate::{slice, sol, FixedBytes, SolType};

sol!(
    struct InitializerInput{
        uint32 height;
        bytes32 header;
        uint64 authoritySetId;
        bytes32 authoritySetHash;
        uint32 headerRangeCommitmentTreeSize;
        bytes vectorProgramVKeyHash;
        bytes vectorProgramVKey;
    }
    struct UpdateFreezeInput{
        bool freeze;
    }
    struct UpdateVectorProgramVkeyInput{
        bytes blobstreamProgramVKeyHash;
        bytes blobstreamProgramVKey;
    }
    struct UpdateCommitmentTreeSizeInput{
        uint32 headerRangeCommitmentTreeSize;
    }
    struct UpdateGenesisStateInput{
        uint32 height;
        bytes32 header;
        uint64 authoritySetId;
        bytes32 authoritySetHash;
    }
    struct UpdateBlockRangeDataInput{
        uint32[] _startBlocks;
        uint32[]  _endBlocks;
        bytes32[]  _headerHashes;
        bytes32[]  _dataRootCommitments;
        bytes32[]  _stateRootCommitments;
        uint64 _endAuthoritySetId;
        bytes32 _endAuthoritySetHash;
    }
    struct CommitHeaderRangeAndRotateInput{
        bytes proof;
        bytes publicValues;
    }
    #[derive(PartialEq)]
    enum ProofType {
        HeaderRangeProof,
        RotateProof
    }
    struct RotateOutputs {
        uint64 current_authority_set_id;
        bytes32 current_authority_set_hash;
        bytes32 new_authority_set_hash;
    }
    struct HeaderRangeOutputs {
        uint32 trusted_block;
        bytes32 trusted_header_hash;
        uint64 authority_set_id;
        bytes32 authority_set_hash;
        uint32 target_block;
        bytes32 target_header_hash;
        bytes32 state_root_commitment;
        bytes32 data_root_commitment;
        uint32 merkle_tree_size;
    }
    struct ProofOutputs {
        ProofType proofType;
        bytes headerRangeOutputs;
        bytes rotateOutputs;
    }
    struct RotateInput{
        bytes proof;
        bytes publicValues;
    }
    struct KeyPacker {
        uint32 latestBlock;
        uint32 targetBlock;
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
        u32,
        Vec<u8>,
        Vec<u8>,
    ) {
        (
            self.height,
            self.header,
            self.authoritySetId,
            self.authoritySetHash,
            self.headerRangeCommitmentTreeSize,
            self.vectorProgramVKeyHash.clone().to_vec(),
            self.vectorProgramVKey.clone().to_vec(),
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

impl UpdateVectorProgramVkeyInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.blobstreamProgramVKeyHash.clone().to_vec(),
            self.blobstreamProgramVKey.clone().to_vec(),
        )
    }
}

impl UpdateCommitmentTreeSizeInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> u32 {
        self.headerRangeCommitmentTreeSize
    }
}

impl UpdateGenesisStateInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (u32, FixedBytes<32>, u64, FixedBytes<32>) {
        (
            self.height,
            self.header.clone(),
            self.authoritySetId,
            self.authoritySetHash.clone(),
        )
    }
}

impl UpdateBlockRangeDataInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(
        &self,
    ) -> (
        Vec<u32>,
        Vec<u32>,
        Vec<FixedBytes<32>>,
        Vec<FixedBytes<32>>,
        Vec<FixedBytes<32>>,
        u64,
        FixedBytes<32>,
    ) {
        (
            self._startBlocks.clone(),
            self._endBlocks.clone(),
            self._headerHashes.clone(),
            self._dataRootCommitments.clone(),
            self._stateRootCommitments.clone(),
            self._endAuthoritySetId,
            self._endAuthoritySetHash.clone(),
        )
    }
}

impl CommitHeaderRangeAndRotateInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.proof.clone().to_vec(),
            self.publicValues.clone().to_vec(),
        )
    }
    pub fn unpack_po(&self) -> (ProofType, Vec<u8>, Vec<u8>) {
        let po = ProofOutputs::abi_decode(&self.publicValues, true).unwrap();
        (
            po.proofType,
            po.headerRangeOutputs.clone().to_vec(),
            po.rotateOutputs.clone().to_vec(),
        )
    }
}

impl HeaderRangeOutputs {
    pub fn new(data: &[u8]) -> Self {
        Self::abi_decode(data, true).unwrap()
    }
    pub fn unpack(
        &self,
    ) -> (
        u32,
        FixedBytes<32>,
        u64,
        FixedBytes<32>,
        u32,
        FixedBytes<32>,
        FixedBytes<32>,
        FixedBytes<32>,
        u32,
    ) {
        (
            self.trusted_block,
            self.trusted_header_hash.clone(),
            self.authority_set_id,
            self.authority_set_hash.clone(),
            self.target_block,
            self.target_header_hash.clone(),
            self.state_root_commitment.clone(),
            self.data_root_commitment.clone(),
            self.merkle_tree_size,
        )
    }
}

impl RotateOutputs {
    pub fn new(data: &[u8]) -> Self {
        Self::abi_decode(data, true).unwrap()
    }
    pub fn unpack(&self) -> (u64, FixedBytes<32>, FixedBytes<32>) {
        (
            self.current_authority_set_id,
            self.current_authority_set_hash.clone(),
            self.new_authority_set_hash.clone(),
        )
    }
}
