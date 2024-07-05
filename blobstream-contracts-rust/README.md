# Blobstream Contracts Rust

| Function name | Function type | Inputs |
|-------------|-----------|------------|
| initializer   | public     | TxContext, InitializerInput|
| update_freeze | public     | TxContext, UpdateFreezeInput |
| commit_header_range | public | TxContext, CommitHeaderRangeInput |
| verify_attestation | public | _, VAInput |
| is_frozen | helper |  |
|is_initialized | helper | |
| msg_sender | helper | TxContext |