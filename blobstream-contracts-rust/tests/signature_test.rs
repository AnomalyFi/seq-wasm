use blobstream_contracts_rust::{signature::is_sig_nil, Signature};

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
