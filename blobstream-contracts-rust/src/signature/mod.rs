use super::{PackerEthSignedMessage, Signature2};
use alloy_primitives::{keccak256, Address, FixedBytes, Signature, B256, B512, U256};
use alloy_sol_types::{abi, SolType, SolValue};
use k256::ecdsa::{Error, RecoveryId, Signature as SignatureK, VerifyingKey};

pub fn is_sig_nil(sig: Signature) -> bool {
    // write tests for this
    //  return (_sig.r == 0 && _sig.s == 0 && _sig.v == 0);
    let zero: U256 = alloy_primitives::U256::from(0);
    let zero_parity = alloy_primitives::Parity::Eip155(0);
    sig.r() == zero && sig.s() == zero && sig.v() == zero_parity
}

pub fn to_eth_signed_message_hash(hash: B256) -> Vec<u8> {
    PackerEthSignedMessage {
        EIP191_SIGNED_MESSAGE: "\x19Ethereum Signed Message:\n".to_string(),
        hash: hash,
    }
    .abi_encode_packed()
}

#[cfg(not(feature = "secp256k1"))]
pub fn ecrecover(sig: &B512, mut recid: u8, msg: &B256) -> Result<B256, Error> {
    // parse signature
    let mut sig = SignatureK::from_slice(sig.as_slice())?;

    // normalize signature and flip recovery id if needed.
    if let Some(sig_normalized) = sig.normalize_s() {
        sig = sig_normalized;
        recid ^= 1;
    }
    let recid = RecoveryId::from_byte(recid).expect("recovery ID is valid");

    // recover key
    let recovered_key = VerifyingKey::recover_from_prehash(&msg[..], &sig, recid)?;
    // hash it
    let mut hash = keccak256(
        &recovered_key
            .to_encoded_point(/* compress = */ false)
            .as_bytes()[1..],
    );

    // truncate to 20 bytes
    hash[..12].fill(0);
    Ok(hash)
}

pub fn verify_sig(mut signer: Address, digest: FixedBytes<32>, sig: Signature2) -> bool {
    let digest_eip191 = to_eth_signed_message_hash(digest);

    let digest_fixed: FixedBytes<32> = FixedBytes::from_slice(&digest_eip191);
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
    use alloy_primitives::{Sign, Signature, U256};
    use sha3::digest::typenum::Zero;

    use super::is_sig_nil;
    // r=0x28ef61340bd939bc2195fe537567866003e1a15d3c71ff63e1590620aa636276
    // v=0x25=37
    // s=0x67cbe9d8997f761aecb703304b3800ccf555c9f3dc64214b297fb1966a3b6d83

    #[test]
    fn test_not_nil_sig() {
        let bytes = [
            40 as u8, 239, 97, 52, 11, 217, 57, 188, 33, 149, 254, 83, 117, 103, 134, 96, 3, 225,
            161, 93, 60, 113, 255, 99, 225, 89, 6, 32, 170, 99, 98, 118, 103, 203, 233, 216, 153,
            127, 118, 26, 236, 183, 3, 48, 75, 56, 0, 204, 245, 85, 201, 243, 220, 100, 33, 75, 41,
            127, 177, 150, 106, 59, 109, 131,
        ];

        let sig = Signature::from_bytes_and_parity(&bytes, 100).unwrap();
        assert_eq!(false, is_sig_nil(sig))
    }
    #[test]
    fn test_nil_sig() {
        let bytes = [0 as u8; 64];
        let sig = Signature::from_bytes_and_parity(&bytes, 0).unwrap();
        let zero: U256 = alloy_primitives::U256::from(0);
        assert_eq!(true, is_sig_nil(sig))
    }
}
