use crate::{input, slice, sol, FixedBytes, SolType, U256};

sol!(
    struct DataRootTuple{
        uint256 height;
        bytes32 dataRoot;
    }
    struct BinaryMerkleProof{
        bytes32[] sideNodes;
        uint256 key;
        uint256 numLeaves;
    }
    struct InitializerInput {
        uint64  height;
        bytes32 header;
        bytes blobstreamProgramVKeyHash;
        bytes blobstreamProgramVKey;
    }
    struct UpdateFreezeInput{
        bool freeze;
    }
    struct UpdateGenesisStateInput{
        uint64 height;
        bytes32 header;
    }
    struct UpdateProgramVkeyInput{
        bytes blobstreamProgramVKeyHash;
        bytes blobstreamProgramVKey;
    }
    struct CommitHeaderRangeInput {
        bytes proof;
        bytes publicValues;
    }
    struct VAInput {
        uint256 tuple_root_nonce;
        DataRootTuple tuple;
        BinaryMerkleProof proof;
    }
    struct ProofOutputs {
        bytes32 trustedHeaderHash;
        bytes32 targetHeaderHash;
        bytes32 dataCommitment;
        uint64 trustedBlock;
        uint64 targetBlock;
        uint256 validatorBitmap;
    }
    struct LeafDigestPacker{
        bytes1 leaf_prefix;
        bytes data;
    }
    struct NodeDigestPacker{
        bytes1 node_prefix;
        bytes32 left;
        bytes32 right;
    }

    struct HelloWorld{
        bytes32 message;
    }
);

impl InitializerInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (u64, FixedBytes<32>, Vec<u8>, Vec<u8>) {
        (
            self.height,
            self.header.clone(),
            self.blobstreamProgramVKeyHash.clone().to_vec(),
            self.blobstreamProgramVKey.clone().to_vec(),
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

impl UpdateGenesisStateInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let init_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(init_input, true).unwrap()
    }
    pub fn unpack(&self) -> (u64, FixedBytes<32>) {
        (self.height, self.header.clone())
    }
}

impl UpdateProgramVkeyInput {
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

impl CommitHeaderRangeInput {
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
    pub fn unpack_po(
        &self,
    ) -> (
        FixedBytes<32>,
        FixedBytes<32>,
        FixedBytes<32>,
        u64,
        u64,
        U256,
    ) {
        let po = ProofOutputs::abi_decode(&self.publicValues, true).unwrap();
        (
            po.trustedHeaderHash,
            po.targetHeaderHash,
            po.dataCommitment,
            po.trustedBlock,
            po.targetBlock,
            po.validatorBitmap,
        )
    }
}
