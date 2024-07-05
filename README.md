# SEQ-WASM Framework

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

- [ ] Call initializer during contract deployment.

- [ ] Change rust memory allocator.

- [x] automate github tests to run on wasm32-unknown-unknown

- [ ] consider caching the abi(initialising abi is taking significant time)

- [ ] Handle unwrap errors. Should the program panic and runtime return error. or the error should be handled by rust program for state access?
