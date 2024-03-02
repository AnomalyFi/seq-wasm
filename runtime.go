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

func Execute() {
	// Choose the context to use for function calls.
	ctxWasm := context.Background()

	// Create a new WebAssembly Runtime.
	r := wazero.NewRuntime(ctxWasm)
	defer r.Close(ctxWasm) // This closes everything this Runtime created.
	_, err := r.NewHostModuleBuilder("env").Instantiate(ctxWasm)
	if err != nil {
		log.Printf("error instantiating module")
	}
	mod, err := r.Instantiate(ctxWasm, wasmBytes)
	if err != nil {
		log.Printf("error instantiating contract")
	}
	allocate_ptr := mod.ExportedFunction("allocate_ptr")
	deallocate_ptr := mod.ExportedFunction("deallocate_ptr")
	// function := ""
	msgSenderLen := 32
	// txFunction := mod.ExportedFunction(function)
	results, err := allocate_ptr.Call(ctxWasm, uint64(msgSenderLen))
	if err != nil {
		log.Printf("error allocating data")
	}
	addressPtr := results[0]
	defer deallocate_ptr.Call(ctxWasm, addressPtr, uint64(msgSenderLen))
}
