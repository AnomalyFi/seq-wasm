# SEQ-WASM Framework

TODO:
- [ ] State revamp

- [x] RPC for fetching values at contract storage slot with rust sdk support.

- [ ] Add tests. 
    - [x] for simple state 
    - [ ] for dynamic state 

- [ ] Fallback mechanisms acting as post-execution hooks in wasm runtime. 

- [ ] Use macros for public functions and state variables.

- [ ] Call initializer during contract deployment.

- [ ] Change rust memory allocator

- [ ] automate github tests to run on wasm32-unknown-unknown

- [ ] consider caching the abi(initialising abi is taking significant time)