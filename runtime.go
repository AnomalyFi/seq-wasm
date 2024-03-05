package runtime

import (
	"context"
	_ "embed"
	"log"

	"github.com/tetratelabs/wazero"
	_ "github.com/tetratelabs/wazero/api"
)

//go:embed blobstream-contracts-rust/target/wasm32-unknown-unknown/release/blobstream_contracts_rust.wasm
var wasmBytes []byte

func Execute(function string, calldata []byte) {
	calldataLen := uint64(len(calldata))
	// Choose the context to use for function calls.
	ctxWasm := context.Background()

	// Create a new WebAssembly Runtime.
	r := wazero.NewRuntime(ctxWasm)
	defer r.Close(ctxWasm) // This closes everything this Runtime created.

	_, err := r.NewHostModuleBuilder("env").Instantiate(ctxWasm)
	if err != nil {
		log.Printf("module instantiation error: %s", err)
	}

	mod, err := r.Instantiate(ctxWasm, wasmBytes)
	if err != nil {
		log.Printf("contract instantiation error: %s", err)
	}
	allocatePtr := mod.ExportedFunction("allocate_ptr")
	deallocatePtr := mod.ExportedFunction("deallocate_ptr")
	txFunc := mod.ExportedFunction(function)
	results, err := allocatePtr.Call(ctxWasm, calldataLen)
	if err != nil {
		log.Printf("data allocation error: %s", err)
	}
	dataPtr := results[0]
	defer deallocatePtr.Call(ctxWasm, dataPtr, calldataLen)

	if !mod.Memory().Write(uint32(dataPtr), calldata) {
		log.Printf("memory write error: %s", err)
	}
	results, err = txFunc.Call(ctxWasm, dataPtr, calldataLen)
	if err != nil {
		log.Printf("function call error: %s", err)
	}

	log.Print("function execution results: ", results)
}
