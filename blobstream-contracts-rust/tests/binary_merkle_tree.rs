use std::vec;

use alloy_primitives::{bytes, fixed_bytes, FixedBytes, U256};
use blobstream_contracts_rust::{binary_merkle_tree::verify, *};
use sha2::Digest;

// #[test]
// fn test_valid_slice() {
//     let data: Vec<FixedBytes<32>> =
//         vec![fixed_bytes!("6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"); 4];
//     let sliced_data = &data[..data.len()];

// }

#[test]
fn test_verify_none() {
    let root = sha2::Sha224::digest("").to_vec();
    let side_nodes = vec![FixedBytes::<32>::new([0; 32])];
    let key = U256::from(0);
    let num_leaves = U256::from(0);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key: key,
        numLeaves: num_leaves,
    };
    let data = bytes!();
    assert_eq!(false, verify(FixedBytes::from_slice(&root), b_m_p, data))
}
