# SEQ-WASM 

WASM smart contracts for SEQ chain.

## Structure:

- [SDK](./sdk/): A frame work for writing smart contracts deployable to SEQ.

- [blobstream-rs](./blobstream-contracts-rust/): A WASM compilable Blobstream version written in rust, compatible with SEQ.

- [vector-rs](./vector-contracts-rust/): A WASM compilable Vector version written in rust, compatible with SEQ.

- [Solidity Gen](./solidity-gen/): Solidity Bindings for function inputs of blobstream-rs, vector-rs and gnark precompile.

- [input tests](./input-types-test/): Tests for state module of sdk, blobstream and vector.

- [vk.bin](./vk.bin): Verification key used for plonk verification in SP1.

TODO:
- [x] State revamp

- [x] RPC for fetching values at contract storage slot with rust sdk support.

- [x] Add tests. 
    - [x] for simple state 
    - [x] for dynamic state 

- [x] Fallbacks getBalance and setBalance for fund dispersion.

- [x] Transfer native token TKN, with empty ID.

- [ ] Use macros for public functions and state variables.

- [x] Blobstream wasm contract size ~110KiB is in limits. If size increases by a large factor, then try optimizing by inlining state functions.

- [x] Call initializer during contract deployment.

- [x] automate github tests to run on wasm32-unknown-unknown

- [x] consider caching the abi(initialising abi is taking significant time)

- [x] Handle unwrap errors. Should the program panic and runtime return error. or the error should be handled by rust program for state access?
