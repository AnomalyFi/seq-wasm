use core::num;

use crate::{BinaryMerkleProof, LeafDigestPacker, NodeDigestPacker};
use alloy_primitives::{hex::FromHex, utils::UnitsError, Bytes, FixedBytes, U256};
use alloy_sol_types::SolValue;
use sha2::{digest, Digest};

// verify:
//      - path_length_from_key
//      - leaf_digest
//      - compute_root_hash

// leaf_digest:
//      - sha256 -> evm's implementation

// path_length_from_key:
//      - get_starting_bit

// compute_root_hash:
//      - get_split_point
//      - slice
//      - node_digest

// get_split_point:
//      - bits_len

pub fn verify(root: FixedBytes<32>, proof: BinaryMerkleProof, data: Bytes) -> bool {
    let one = U256::from(1);
    if proof.numLeaves <= one {
        if proof.sideNodes.len() != 0 {
            if !(proof.sideNodes.len() == 1
                && proof.sideNodes == vec![FixedBytes::<32>::new([0; 32])])
            {
                return false; // @todo should we include error codes?
            }
        }
    }
    // lol fixed_bytes of side nodes, need to be validated in a different way
    if U256::from(proof.sideNodes.len()) != path_length_from_key(proof.key, proof.numLeaves) {
        return false;
    }
    // check if key is in tree
    if proof.key >= proof.numLeaves {
        return false;
    }
    let digest = leaf_digest(data);

    // Null proof is valid if num_leaves = 1, if so just veify hash(data) is root
    if proof.sideNodes.len() == 0 {
        if proof.numLeaves == one {
            return root == digest;
        }

        return false;
    }
    let (hash, is_error) = compute_root_hash(proof.key, proof.numLeaves, digest, proof.sideNodes);
    if is_error {
        println!("heyo is_error");
        return false;
    }

    hash == root
}

pub fn path_length_from_key(key: U256, num_leaves: U256) -> U256 {
    let max_height = U256::from(256);
    let one = U256::from(1);
    if num_leaves <= one {
        return U256::from(0);
    }
    let path_length = max_height - get_starting_bit(num_leaves);
    let num_leaves_left_sub_tree = one << (path_length - one);
    if key <= num_leaves_left_sub_tree - one {
        path_length
    } else if num_leaves_left_sub_tree == one {
        one
    } else {
        one + path_length_from_key(
            key - num_leaves_left_sub_tree,
            num_leaves - num_leaves_left_sub_tree,
        )
    }
}

pub fn get_starting_bit(num_leaves: U256) -> U256 {
    let max_height = U256::from(256);
    let one = U256::from(1);
    let mut starting_bit = U256::from(0);
    while (one << starting_bit) < num_leaves {
        starting_bit += one;
    }
    max_height - starting_bit
}

pub fn leaf_digest(data: Bytes) -> FixedBytes<32> {
    let leaf_prefix = FixedBytes::<1>::from_hex("00").unwrap();
    let leaf_digest = LeafDigestPacker {
        leaf_prefix,
        data: data.to_vec(),
    };
    let digest = sha2::Sha256::digest(leaf_digest.abi_encode_packed()).to_vec();
    FixedBytes::from_slice(&digest)
}

pub fn compute_root_hash(
    key: U256,
    num_leaves: U256,
    leaf_hash: FixedBytes<32>,
    side_nodes: Vec<FixedBytes<32>>,
) -> (FixedBytes<32>, bool /* is_error */) {
    let zero = U256::from(0);
    let one = U256::from(1);
    if num_leaves == zero {
        return (leaf_hash, true);
    }
    if num_leaves == one {
        if side_nodes.len() != 0 {
            return (leaf_hash, true);
        }
        return (leaf_hash, false);
    }
    if side_nodes.len() == 0 {
        return (leaf_hash, true);
    }
    let num_left = get_split_point(num_leaves);
    let side_nodes_left = &side_nodes[..side_nodes.len() - 1]; //@todo doubtful here
    if key < num_left {
        let (left_hash, is_error) =
            compute_root_hash(key, num_left, leaf_hash, side_nodes_left.to_vec());
        if is_error {
            return (leaf_hash, is_error);
        }
        return (
            node_digest(left_hash, side_nodes[side_nodes.len() - 1]),
            false,
        );
    }
    let (right_hash, is_error) = compute_root_hash(
        key - num_left,
        num_leaves - num_left,
        leaf_hash,
        side_nodes_left.to_vec(),
    );
    if is_error {
        return (leaf_hash, is_error);
    }

    (
        node_digest(side_nodes[side_nodes.len() - 1], right_hash),
        false,
    )
}

/// returns largest power of 2 less than x
pub fn get_split_point(x: U256) -> U256 {
    let one = U256::from(1);
    if x < one {
        panic!(); // TODO: panic or false
    }
    let bit_len = bits_len(x);
    let mut k = one << (bit_len - one);
    if k == x {
        k >>= one;
    }
    k
}

pub fn bits_len(mut x: U256) -> U256 {
    let zero = U256::from(0);
    let one = U256::from(1);
    let mut count = zero;
    while x != zero {
        count += one;
        x >>= one;
    }
    count
}

//@todo translate math libraries in a better way here.

pub fn node_digest(left: FixedBytes<32>, right: FixedBytes<32>) -> FixedBytes<32> {
    let node_prefix = FixedBytes::<1>::from_hex("01").unwrap();
    let node_digest = NodeDigestPacker {
        node_prefix,
        left,
        right,
    }
    .abi_encode_packed();
    let digest = sha2::Sha256::digest(node_digest).to_vec();
    FixedBytes::from_slice(&digest)
}

// pub fn slice(data: Vec<FixedBytes<32>>, begin: U256, end: U256) -> Vec<FixedBytes<32>> {
//     if begin > end {
//         panic!(); // panic or false
//     }
//     if begin > U256::from(data.len()) || end > U256::from(data.len()) {
//         panic!();
//     }
//     let out: Vec<FixedBytes<32>> = Vec::new();
//     let i = U256::from(begin);
//     while
// }

#[cfg(test)]
mod test_binary_merkle_tree {
    use super::{leaf_digest, node_digest};
    use alloy_primitives::{bytes, fixed_bytes};
    #[test]
    fn test_leaf_digest_empty() {
        let expected =
            fixed_bytes!("6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        assert_eq!(leaf_digest(bytes!()), expected)
    }
    #[test]
    fn test_leaf_digest_with_some_data() {
        let expected =
            fixed_bytes!("48c90c8ae24688d6bef5d48a30c2cc8b6754335a8db21793cc0a8e3bed321729");
        assert_eq!(leaf_digest(bytes!("deadbeef")), expected)
    }
    #[test]
    fn test_node_digest_empty_children() {
        let expected =
            fixed_bytes!("fe43d66afa4a9a5c4f9c9da89f4ffb52635c8f342e7ffb731d68e36c5982072a");
        let left = fixed_bytes!("6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        let right =
            fixed_bytes!("6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d");
        assert_eq!(node_digest(left, right), expected);
    }
    #[test]
    fn test_node_digest_some_children() {
        let expected =
            fixed_bytes!("62343bba7c4d6259f0d4863cdf476f1c0ac1b9fbe9244723a9b8b5c8aae72c38");
        let left = fixed_bytes!("db55da3fc3098e9c42311c6013304ff36b19ef73d12ea932054b5ad51df4f49d");
        let right =
            fixed_bytes!("c75cb66ae28d8ebc6eded002c28a8ba0d06d3a78c6b5cbf9b2ade051f0775ac4");
        assert_eq!(node_digest(left, right), expected)
    }
}
