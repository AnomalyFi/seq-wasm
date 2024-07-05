# SEQ-WASM Framework

TODO:
- [x] State revamp

- [x] RPC for fetching values at contract storage slot with rust sdk support.

- [x] Add tests. 
    - [x] for simple state 
    - [x] for dynamic state 

- [ ] Fallback mechanisms for fund dispersion. 

- [ ] Use macros for public functions and state variables.

- [x] Blobstream wasm contract size ~110KiB is in limits. If size increases by a large factor, then try optimizing by inlining state functions.

- [ ] Handle unwrap errors.

- [ ] Call initializer during contract deployment.

- [ ] Change rust memory allocator.

- [ ] automate github tests to run on wasm32-unknown-unknown

- [ ] consider caching the abi(initialising abi is taking significant time)
