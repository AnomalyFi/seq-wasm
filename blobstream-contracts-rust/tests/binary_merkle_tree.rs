use core::num;
use std::vec;

use alloy_primitives::{bytes, fixed_bytes, FixedBytes, U256};
use blobstream_contracts_rust::{binary_merkle_tree::verify, *};
// use sha2::Digest;

fn set_up() -> BinaryMerkleProof {
    let side_nodes = vec![FixedBytes::<32>::new([0; 32])];
    let key = U256::from(0);
    let num_leaves = U256::from(1);
    BinaryMerkleProof {
        sideNodes: side_nodes,
        key: key,
        numLeaves: num_leaves,
    }
}

#[test]
fn test_verify_none() {
    // let sha_hash = sha2::Sha224::digest("").to_vec();
    let root = fixed_bytes!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    let side_nodes = vec![FixedBytes::<32>::new([0; 32])];
    let key = U256::from(0);
    let num_leaves = U256::from(0);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key: key,
        numLeaves: num_leaves,
    };
    let data = bytes!();
    assert_eq!(false, verify(root, b_m_p, data))
}
//@todo
#[test]
fn test_verify_one_leaf_empty() {
    let root = fixed_bytes!("6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
    let b_m_p = set_up();
    let data = bytes!();
    assert_eq!(true, verify(root, b_m_p, data));
}
//@todo
#[test]
fn test_verify_one_leaf_some() {
    let root = fixed_bytes!("48c90c8ae24688d6bef5d48a30c2cc8b6754335a8db21793cc0a8e3bed321729");
    let b_m_p = set_up();
    let data = bytes!("deadbeef");
    assert_eq!(true, verify(root, b_m_p, data));
}
//@todo
#[test]
fn test_verify_one_leaf_01() {
    let root = fixed_bytes!("b413f47d13ee2fe6c845b2ee141af81de858df4ec549a58b7970bb96645bc8d2");
    let b_m_p = set_up();
    let data = bytes!("01");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_leaf_one_of_eight() {
    let root = fixed_bytes!("c1ad6548cb4c7663110df219ec8b36ca63b01158956f4be31a38a88d0c7f7071");
    let side_nodes = vec![
        fixed_bytes!("fcf0a6c700dd13e274b6fba8deea8dd9b26e4eedde3495717cac8408c9c5177f"),
        fixed_bytes!("78850a5ab36238b076dd99fd258c70d523168704247988a94caa8c9ccd056b8d"),
        fixed_bytes!("4301a067262bbb18b4919742326f6f6d706099f9c0e8b0f2db7b88f204b2cf09"),
    ];
    let key = U256::from(0);
    let num_leaves = U256::from(8);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_leaf_two_of_eight() {
    let root = fixed_bytes!("c1ad6548cb4c7663110df219ec8b36ca63b01158956f4be31a38a88d0c7f7071");
    let side_nodes = vec![
        fixed_bytes!("b413f47d13ee2fe6c845b2ee141af81de858df4ec549a58b7970bb96645bc8d2"),
        fixed_bytes!("78850a5ab36238b076dd99fd258c70d523168704247988a94caa8c9ccd056b8d"),
        fixed_bytes!("4301a067262bbb18b4919742326f6f6d706099f9c0e8b0f2db7b88f204b2cf09"),
    ];
    let key = U256::from(1);
    let num_leaves = U256::from(8);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("02");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_leaf_three_of_eight() {
    let root = fixed_bytes!("c1ad6548cb4c7663110df219ec8b36ca63b01158956f4be31a38a88d0c7f7071");
    let side_nodes = vec![
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
        fixed_bytes!("6bcf0e2e93e0a18e22789aee965e6553f4fbe93f0acfc4a705d691c8311c4965"),
        fixed_bytes!("4301a067262bbb18b4919742326f6f6d706099f9c0e8b0f2db7b88f204b2cf09"),
    ];
    let key = U256::from(2);
    let num_leaves = U256::from(8);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("03");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_seven_of_eight() {
    let root = fixed_bytes!("c1ad6548cb4c7663110df219ec8b36ca63b01158956f4be31a38a88d0c7f7071");
    let side_nodes = vec![
        fixed_bytes!("b4c43b50bf245bd727623e3c775a8fcfb8d823d00b57dd65f7f79dd33f126315"),
        fixed_bytes!("90eeb2c4a04ec33ee4dd2677593331910e4203db4fcc120a6cdb95b13cfe83f0"),
        fixed_bytes!("fa02d31a63cc11cc624881e52af14af7a1c6ab745efa71021cb24086b9b1793f"),
    ];
    let key = U256::from(6);
    let num_leaves = U256::from(8);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("07");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_eight_of_eight() {
    let root = fixed_bytes!("c1ad6548cb4c7663110df219ec8b36ca63b01158956f4be31a38a88d0c7f7071");
    let side_nodes = vec![
        fixed_bytes!("2ecd8a6b7d2845546659ad4cf443533cf921b19dc81fa83934e83821b4dfdcb7"),
        fixed_bytes!("90eeb2c4a04ec33ee4dd2677593331910e4203db4fcc120a6cdb95b13cfe83f0"),
        fixed_bytes!("fa02d31a63cc11cc624881e52af14af7a1c6ab745efa71021cb24086b9b1793f"),
    ];
    let key = U256::from(7);
    let num_leaves = U256::from(8);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("08");
    assert_eq!(true, verify(root, b_m_p, data))
}

// Test vectors:
// 0x00
// 0x01
// 0x02
// 0x03
// 0x04
#[test]
fn test_verify_proof_of_five_leaves() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
    ];
    let key = U256::from(1);
    let num_leaves = U256::from(5);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(true, verify(root, b_m_p, data))
}

#[test]
fn test_verify_invalid_proof_root() {
    // correct root: 0xb855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f;
    let root = fixed_bytes!("c855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
    ];
    let key = U256::from(1);
    let num_leaves = U256::from(5);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(false, verify(root, b_m_p, data))
}

#[test]
fn test_verify_invalid_proof_key() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
    ];
    // correct key: 1
    let key = U256::from(2);
    let num_leaves = U256::from(5);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(false, verify(root, b_m_p, data))
}

#[test]
fn test_verify_invalid_proof_number_of_leaves() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
    ];
    let key = U256::from(1);
    // currect num_leaves = 5
    let num_leaves = U256::from(100);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(false, verify(root, b_m_p, data))
}

#[test]
fn test_verify_invalid_proof_side_nodes() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("5f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"), // correct side node: 4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4
    ];
    let key = U256::from(1);
    let num_leaves = U256::from(5);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(false, verify(root, b_m_p, data))
}

#[test]
fn test_verify_invalid_proof_data() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![
        fixed_bytes!("96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7"),
        fixed_bytes!("52c56b473e5246933e7852989cd9feba3b38f078742b93afff1e65ed46797825"),
        fixed_bytes!("4f35212d12f9ad2036492c95f1fe79baf4ec7bd9bef3dffa7579f2293ff546a4"),
    ];
    let key = U256::from(1);
    let num_leaves = U256::from(5);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("012345"); // correct data: 01
    assert_eq!(false, verify(root, b_m_p, data))
}

#[test]
fn test_same_key_and_leaves_number() {
    let root = fixed_bytes!("b855b42d6c30f5b087e05266783fbd6e394f7b926013ccaa67700a8b0c5a596f");
    let side_nodes = vec![FixedBytes::<32>::new([0; 32])];
    let key = U256::from(3);
    let num_leaves = U256::from(3);
    let b_m_p = BinaryMerkleProof {
        sideNodes: side_nodes,
        key,
        numLeaves: num_leaves,
    };
    let data = bytes!("01");
    assert_eq!(false, verify(root, b_m_p, data));
}
