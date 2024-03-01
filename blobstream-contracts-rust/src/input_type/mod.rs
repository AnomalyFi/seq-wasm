use super::{BinaryMerkleProof, DataRootTuple, FixedBytes, Layout, Signature, Validator, U256};
pub struct UVSInput {
    new_nonce: U256,
    old_nonce: U256,
    new_power_threshold: U256,
    new_validator_set_hash: FixedBytes<32>,
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
}

pub struct SDRTRInput {
    new_nonce: U256,
    validator_set_nonce: U256,
    data_root_tuple_root: FixedBytes<32>,
    current_validators: Vec<Validator>,
    signatures: Vec<Signature>,
}

pub struct VAInput {
    tuple_root_nonce: U256,
    tuple: DataRootTuple,
    proof: BinaryMerkleProof,
}

impl UVSInput {
    pub fn new(ptr: *const UVSInput) -> Self {
        let uvs_input = unsafe { &*ptr };
        UVSInput {
            new_nonce: uvs_input.new_nonce,
            old_nonce: uvs_input.new_nonce,
            new_power_threshold: uvs_input.new_power_threshold,
            new_validator_set_hash: uvs_input.new_validator_set_hash,
            current_validators: uvs_input.current_validators.clone(),
            signatures: uvs_input.signatures.clone(),
        }
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
    fn pack_to() {}
    fn size_of() {}
    fn to_bytes() {}
}

impl SDRTRInput {
    pub fn new(ptr: *const SDRTRInput) -> Self {
        let sdrtr_input = unsafe { &*ptr };
        SDRTRInput {
            new_nonce: sdrtr_input.new_nonce,
            validator_set_nonce: sdrtr_input.validator_set_nonce,
            data_root_tuple_root: sdrtr_input.data_root_tuple_root,
            current_validators: sdrtr_input.current_validators.clone(),
            signatures: sdrtr_input.signatures.clone(),
        }
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
    pub fn new(ptr: *const VAInput) -> Self {
        let va_input = unsafe { &*ptr };
        VAInput {
            tuple_root_nonce: va_input.tuple_root_nonce,
            tuple: va_input.tuple.clone(),
            proof: va_input.proof.clone(),
        }
    }
    pub fn unpack(&self) -> (U256, DataRootTuple, BinaryMerkleProof) {
        (
            self.tuple_root_nonce,
            self.tuple.clone(),
            self.proof.clone(),
        )
    }
}
