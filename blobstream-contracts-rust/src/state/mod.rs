#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "stateStoreBytes"]
    pub fn store_bytes(slot: u32, ptr: u32, size: u32);
    #[link_name = "stateGetBytes"]
    pub fn get_bytes(slot: u32) -> u64;
}

// use enums for state variables & provide enough abstraction

// Storage layout:
// # storage keys: 1024(default) --> # of storage keys accesible by the contract
// # static keys: 128(default) --> # of storage keys allocated for static varialbes(Uint256, Bytes32...)
// # dynamic keys: 896(default) --> # of storage keys allocated for dynamic types --> mapping, arrays --> after 896 keys are utilised, keys are wrapped to 129 again(clock arithmatic)
// # of storage keys and allocation of storage keys are set during contract deployement. Can be modified by SEQ midway on necessary(ex: when celestia is out)

// for celestia's blobstream case:
// 3 static variables --> state_lastValidatorSetCheckPoint, state_powerThereshold, state_eventNonce
// 1 dynamic variable type --> mapping(uint256 => bytes32) state_dataRootTupleRoots
// this encourages validators to post blocks as early as possible.

// check how frequently celesia posts block roots to ethereum??
// @todo
