use super::{PackerEthSignedMessage, Signature};
use alloy_primitives::{keccak256, Address, Bytes, FixedBytes, B256, B512};
use alloy_sol_types::SolValue;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, Secp256k1,
};

pub fn is_sig_nil(sig: &Signature) -> bool {
    let zero_fb_32 = FixedBytes::<32>::with_last_byte(0);
    sig.r == zero_fb_32 && sig.s == zero_fb_32 && sig.v == 0
}

pub fn to_eth_signed_message_hash(hash: Bytes) -> Vec<u8> {
    PackerEthSignedMessage {
        EIP191_SIGNED_MESSAGE: "\x19Ethereum Signed Message:\n".to_string(),
        len: hash.len().to_string(),
        hash: hash.to_vec(),
    }
    .abi_encode_packed()
}

pub fn ecrecover(sig: &B512, recid: u8, msg: &B256) -> Result<B256, secp256k1::Error> {
    let recid = RecoveryId::from_i32(recid as i32).expect("recovery ID is valid");
    let sig = RecoverableSignature::from_compact(sig.as_slice(), recid)?;

    let secp = Secp256k1::new();
    let msg = Message::from_digest_slice(msg.as_slice())?;
    let public = secp.recover_ecdsa(&msg, &sig)?;

    let mut hash = keccak256(&public.serialize_uncompressed()[1..]);
    hash[..12].fill(0);
    Ok(hash)
}

pub fn verify_sig(signer: Address, digest: Bytes, sig: &Signature) -> bool {
    let digest_eip191 = to_eth_signed_message_hash(digest);

    let digest_fixed = keccak256(digest_eip191);
    let v = sig.v - 27;
    if v != 0 && v != 1 {
        return false;
    }
    // confirm is r and s are packed as r:s
    let r = FixedBytes::<64>::right_padding_from(&sig.r.to_vec());
    let s = FixedBytes::<64>::left_padding_from(&sig.s.to_vec());
    let sig_packed = r.bit_or(s);
    let recovered_address = ecrecover(&sig_packed, v, &digest_fixed).unwrap();
    let signer_converted = FixedBytes::<32>::left_padding_from(&signer.to_vec());

    recovered_address == signer_converted
}
#[cfg(test)]
mod tests {
    use super::{is_sig_nil, verify_sig};
    use crate::Signature;
    use alloy_primitives::{address, bytes, fixed_bytes, FixedBytes};

    #[test]
    fn test_not_nil_sig() {
        let r = &[
            40 as u8, 239, 97, 52, 11, 217, 57, 188, 33, 149, 254, 83, 117, 103, 134, 96, 3, 225,
            161, 93, 60, 113, 255, 99, 225, 89, 6, 32, 170, 99, 98, 118,
        ];
        let s = &[
            103 as u8, 203, 233, 216, 153, 127, 118, 26, 236, 183, 3, 48, 75, 56, 0, 204, 245, 85,
            201, 243, 220, 100, 33, 75, 41, 127, 177, 150, 106, 59, 109, 131,
        ];

        let sig = Signature {
            v: 27,
            r: FixedBytes::from_slice(r),
            s: FixedBytes::from_slice(s),
        };
        assert_eq!(false, is_sig_nil(&sig))
    }
    #[test]
    fn test_nil_sig() {
        let r = &[0 as u8; 32];
        let sig = Signature {
            v: 0,
            r: r.into(),
            s: r.into(),
        };
        assert_eq!(true, is_sig_nil(&sig))
    }
    #[test]
    fn test_verify_valid_sig() {
        let signer = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        let digest = bytes!("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e");
        let v: u8 = 27;
        let r = fixed_bytes!("5ed039605f2ff0b5431d47b951e0f6868686d7a41541da9810c5d93eaf34251e");
        let s = fixed_bytes!("7752efe4b8c794762181ad152f86fd2d7fecbf444b392bfde90d6fad1e79695f");
        let sig = Signature { v: v, r: r, s: s };
        assert_eq!(true, verify_sig(signer, digest, &sig));
    }
    #[test]
    fn test_wrong_signer() {
        let signer = address!("35AeEb1cAc11D7C084e65F3217f111dA03493bB1");
        let digest = bytes!("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e");
        let v: u8 = 27;
        let r = fixed_bytes!("5ed039605f2ff0b5431d47b951e0f6868686d7a41541da9810c5d93eaf34251e");
        let s = fixed_bytes!("7752efe4b8c794762181ad152f86fd2d7fecbf444b392bfde90d6fad1e79695f");
        let sig = Signature { v: v, r: r, s: s };
        assert_eq!(false, verify_sig(signer, digest, &sig));
    }
    #[test]
    fn test_wrong_sig_v() {
        let signer = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        let digest = bytes!("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e");
        let v: u8 = 28;
        let r = fixed_bytes!("5ed039605f2ff0b5431d47b951e0f6868686d7a41541da9810c5d93eaf34251e");
        let s = fixed_bytes!("7752efe4b8c794762181ad152f86fd2d7fecbf444b392bfde90d6fad1e79695f");
        let sig = Signature { v: v, r: r, s: s };
        assert_eq!(false, verify_sig(signer, digest, &sig));
    }
    #[test]
    fn test_wrong_digest() {
        let signer = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        let digest = bytes!("86d0ff68891491bd7edc2a1f2eec35ab473cc6d55bef94c4388d76a15e456ad2");
        let v: u8 = 27;
        let r = fixed_bytes!("5ed039605f2ff0b5431d47b951e0f6868686d7a41541da9810c5d93eaf34251e");
        let s = fixed_bytes!("7752efe4b8c794762181ad152f86fd2d7fecbf444b392bfde90d6fad1e79695f");
        let sig = Signature { v: v, r: r, s: s };
        assert_eq!(false, verify_sig(signer, digest, &sig));
    }
    #[test]
    fn test_valid_dsvsd_sig() {
        //check for domain seperator validator data signature to be valid ? next constuct the digest from our tooling @todo
        let digest = bytes!("41e6dab883d55f97064a93fda5054aa5ba6ef1b1d41c55f608dac0242c924efc");
        let signer = address!("9c2B12b5a07FC6D719Ed7646e5041A7E85758329");
        let v: u8 = 27;
        let r = fixed_bytes!("02bd9e5fe41ca09e69c688eb127ba3a710ba0f9f9080b13c1f003126a74be2d5");
        let s = fixed_bytes!("6dc6943fc93d17984e3ac3023b15030b33a5c9b6e647ddfb3a7f19a1c3ce9a2e");
        let sig = Signature { v, r, s };
        assert_eq!(true, verify_sig(signer, digest, &sig))
    }
}
