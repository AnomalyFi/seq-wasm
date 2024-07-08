// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract SolGen {
    struct gnarkPrecompileInputs{
        bytes programVKeyHash;
        bytes publicValues;
        bytes proofBytes;
        bytes programVKey;
    }

    function gnarkPrecompile(gnarkPrecompileInputs calldata inputs) public returns (bool) {
        return true;
    }
}