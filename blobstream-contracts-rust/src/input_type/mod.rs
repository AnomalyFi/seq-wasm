use core::slice;

use alloy_sol_types::SolType;

use super::{sol, BinaryMerkleProof, DataRootTuple, FixedBytes, Signature, Validator, U256};

sol!(
    struct SDRTRInput {
        uint256 new_nonce;
        uint256 validator_set_nonce;
        bytes32 data_root_tuple_root;
        Validator[] current_validators;
        Signature[] signatures;
    }
    struct UVSInput {
        uint256 new_nonce;
        uint256 old_nonce;
        uint256 new_power_threshold;
        bytes32 new_validator_set_hash;
        Validator[] current_validators;
        Signature[] signatures;
    }
    struct VAInput {
        uint256 tuple_root_nonce;
        DataRootTuple tuple;
        BinaryMerkleProof proof;
    }
);

impl UVSInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let sdrtr_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(sdrtr_input, true).unwrap()
    }
    pub fn unpack(
        &self,
    ) -> (
        U256,
        U256,
        U256,
        FixedBytes<32>,
        Vec<Validator>,
        Vec<Signature>,
    ) {
        (
            self.new_nonce,
            self.old_nonce,
            self.new_power_threshold,
            self.new_validator_set_hash,
            self.current_validators.clone(),
            self.signatures.clone(),
        )
    }
}

impl SDRTRInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let sdrtr_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(sdrtr_input, true).unwrap()
    }
    pub fn unpack(&self) -> (U256, U256, FixedBytes<32>, Vec<Validator>, Vec<Signature>) {
        (
            self.new_nonce,
            self.validator_set_nonce,
            self.data_root_tuple_root,
            self.current_validators.clone(),
            self.signatures.clone(),
        )
    }
}
impl VAInput {
    pub fn new(ptr: *const u8, len: u32) -> Self {
        let sdrtr_input = unsafe { slice::from_raw_parts(ptr, (len as u16).into()) };
        Self::abi_decode(sdrtr_input, true).unwrap()
    }
    pub fn unpack(&self) -> (U256, DataRootTuple, BinaryMerkleProof) {
        (
            self.tuple_root_nonce,
            self.tuple.clone(),
            self.proof.clone(),
        )
    }
}
