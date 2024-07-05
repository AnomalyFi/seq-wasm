use seq_wasm_sdk::types::*;

#[test]
fn test_address_correct_len() {
    let a_b = vec![0u8; Address::LEN];
    let address = Address::new(a_b.try_into().unwrap());
    assert_eq!(address.as_bytes(), &[0; Address::LEN]);
}

#[test]
fn test_id_correct_len() {
    let a_b = vec![0u8; ID::LEN];
    let id = ID::new(a_b.try_into().unwrap());
    assert_eq!(id.as_bytes(), &[0; ID::LEN]);
}

#[test]
fn test_empty_id() {
    let id = ID::empty_id();
    assert_eq!(id.as_bytes(), &[0; ID::LEN]);
}
