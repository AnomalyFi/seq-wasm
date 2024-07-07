// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

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

struct DataRootTuple{
        uint256 height;
        bytes32 dataRoot;
}
struct BinaryMerkleProof{
    bytes32[] sideNodes;
    uint256 key;
    uint256 numLeaves;
}

interface BlobStreamInputs {
    function initializer(InitializerInput calldata inputs) external; 
    function updateFreeze(UpdateFreezeInput calldata inputs) external; 
    function updateGenesisState(UpdateGenesisStateInput calldata inputs) external;  
    function updateProgramVkey(UpdateProgramVkeyInput calldata inputs) external;  
    function commitHeaderRange(CommitHeaderRangeInput calldata inputs) external;  
    function verifyAppend(VAInput calldata inputs) external;
}