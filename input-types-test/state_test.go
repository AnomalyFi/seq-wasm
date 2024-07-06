package main

import (
	"context"
	"encoding/hex"
	"io/ioutil"
	"os"
	"strconv"
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
)

func TestState(t *testing.T) {
	wasmByte, _ := ioutil.ReadFile("/home/ubuntu/seq-wasm/target/wasm32-unknown-unknown/release/input_types_test.wasm")

	ctxWasm := context.Background()
	mapper := map[string][]byte{
		"0": {0, 1},
	}

	mod /*allocate_ptr*/, _, err := runtime(ctxWasm, mapper, wasmByte)
	require.NoError(t, err)
	test_store_u256 := mod.ExportedFunction("test_store_u256")
	test_get_u256 := mod.ExportedFunction("test_get_u256")
	test_store_u64 := mod.ExportedFunction("test_store_u64")
	test_get_u64 := mod.ExportedFunction("test_get_u64")
	test_store_u32 := mod.ExportedFunction("test_store_u32")
	test_get_u32 := mod.ExportedFunction("test_get_u32")
	test_store_bool := mod.ExportedFunction("test_store_bool")
	test_get_bool := mod.ExportedFunction("test_get_bool")
	test_store_bytes32 := mod.ExportedFunction("test_store_bytes32")
	test_get_bytes32 := mod.ExportedFunction("test_get_bytes32")
	test_store_bytes := mod.ExportedFunction("test_store_bytes")
	test_get_bytes := mod.ExportedFunction("test_get_bytes")
	test_store_mapping_u256_bytes32 := mod.ExportedFunction("test_store_mapping_u256_bytes32")
	test_get_mapping_u256_bytes32 := mod.ExportedFunction("test_get_mapping_u256_bytes32")
	test_store_mapping_u64_bytes32 := mod.ExportedFunction("test_store_mapping_u64_bytes32")
	test_get_mapping_u64_bytes32 := mod.ExportedFunction("test_get_mapping_u64_bytes32")
	test_store_mapping_u32_bytes32 := mod.ExportedFunction("test_store_mapping_u32_bytes32")
	test_get_mapping_u32_bytes32 := mod.ExportedFunction("test_get_mapping_u32_bytes32")
	test_store_mapping_bytes32_bytes32 := mod.ExportedFunction("test_store_mapping_bytes32_bytes32")
	test_get_mapping_bytes32_bytes32 := mod.ExportedFunction("test_get_mapping_bytes32_bytes32")
	test_store_mapping_bytes32_u32 := mod.ExportedFunction("test_store_mapping_bytes32_u32")
	test_get_mapping_bytes32_u32 := mod.ExportedFunction("test_get_mapping_bytes32_u32")
	test_precompiles_module_call := mod.ExportedFunction("test_precompiles_module_call")
	test_multi_input := mod.ExportedFunction("test_multi_input")
	// store and get u256
	_, err = test_store_u256.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123}, mapper["slot1"])
	result, err := test_get_u256.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get u64
	_, err = test_store_u64.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{0, 0, 0, 0, 0, 0, 0, 123}, mapper["slot3"])
	result, err = test_get_u64.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get u32
	_, err = test_store_u32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{0, 0, 0, 123}, mapper["slot5"])
	result, err = test_get_u32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get bool
	_, err = test_store_bool.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{0, 0, 0, 1}, mapper["slot7"])
	result, err = test_get_bool.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get bytes32
	_, err = test_store_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, mapper["slot9"])
	result, err = test_get_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get bytes
	_, err = test_store_bytes.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, []byte{1, 2, 3}, mapper["slot11"])
	result, err = test_get_bytes.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get mapping u256 bytes32
	_, err = test_store_mapping_u256_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	slot := "slot" + strconv.Itoa(1) + hex.EncodeToString([]byte{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 57})
	require.Equal(t, []byte{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, mapper[slot])
	result, err = test_get_mapping_u256_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get mapping u64 bytes32
	_, err = test_store_mapping_u64_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	slot = "slot" + strconv.Itoa(3) + hex.EncodeToString([]byte{0, 0, 0, 0, 0, 0, 16, 134})
	require.Equal(t, []byte{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, mapper[slot])
	result, err = test_get_mapping_u64_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get mapping u32 bytes32
	_, err = test_store_mapping_u32_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	slot = "slot" + strconv.Itoa(5) + hex.EncodeToString([]byte{0, 0, 0, 123})
	require.Equal(t, []byte{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}, mapper[slot])
	result, err = test_get_mapping_u32_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get mapping bytes32 bytes32
	_, err = test_store_mapping_bytes32_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	slot = "slot" + strconv.Itoa(7) + hex.EncodeToString([]byte{2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2})
	require.Equal(t, []byte{4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4}, mapper[slot])
	result, err = test_get_mapping_bytes32_bytes32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	// store and get mapping bytes32 u32
	_, err = test_store_mapping_bytes32_u32.Call(ctxWasm)
	require.NoError(t, err)
	slot = "slot" + strconv.Itoa(9) + hex.EncodeToString([]byte{2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2})
	require.Equal(t, []byte{0, 0, 48, 81}, mapper[slot])
	result, err = test_get_mapping_bytes32_u32.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	_, err = test_precompiles_module_call.Call(ctxWasm)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(1))

	result, err = test_multi_input.Call(ctxWasm, 1, 2, 4, 5)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(12))
}

func runtime(ctxWasm context.Context, mapper map[string][]byte, wasmByte []byte) (api.Module, api.Function, error) {

	var allocate_ptr api.Function
	r := wazero.NewRuntime(ctxWasm)

	stateStoreBytesInner := func(ctxInner context.Context, m api.Module, i uint32, ptr uint32, size uint32) {
		slot := "slot" + strconv.Itoa(int(i))
		bytes, ok := m.Memory().Read(ptr, size)
		if !ok {
			os.Exit(10)
		}
		mapper[slot] = bytes
	}
	stateGetBytesInner := func(ctxInner context.Context, m api.Module, i uint32) uint64 {
		slot := "slot" + strconv.Itoa(int(i))
		result := mapper[slot]
		size := uint64(len(result))
		results, _ := allocate_ptr.Call(ctxInner, size)
		offset := results[0]
		m.Memory().Write(uint32(offset), result)
		return uint64(offset)<<32 | size
	}
	stateStoreDynamicBytesInner := func(ctxInner context.Context, m api.Module, id, ptrKey, sizeOfKey, ptr, size uint32) {
		// read key from memory.
		key, ok := m.Memory().Read(ptrKey, sizeOfKey)
		if !ok {
			os.Exit(10)
		}
		// read value from memory.
		bytes, ok := m.Memory().Read(ptr, size)
		if !ok {
			os.Exit(10)
		}
		slot := "slot" + strconv.Itoa(int(id)) + hex.EncodeToString(key)
		mapper[slot] = bytes
	}
	stateGetDynamicBytesInner := func(ctxInner context.Context, m api.Module, id, ptrKey, sizeOfKey uint32) uint64 {
		// read key from memory.
		key, ok := m.Memory().Read(ptrKey, sizeOfKey)
		if !ok {
			os.Exit(10)
		}
		slot := "slot" + strconv.Itoa(int(id)) + hex.EncodeToString(key)
		result := mapper[slot]
		// write value to memory.
		size := uint64(len(result))
		results, _ := allocate_ptr.Call(ctxInner, size)
		offset2 := results[0]
		m.Memory().Write(uint32(offset2), result)
		return uint64(offset2)<<32 | size
	}
	gnarkVerify := func(ctxInner context.Context, m api.Module, ptr uint32, size uint32) uint32 {
		return 1
	}

	addBalance := func(ctxInner context.Context, m api.Module) {
		// storage.AddBalance()
	}
	subBalance := func(ctxInner context.Context, m api.Module) {}

	// Instantiate the module
	_, err := r.NewHostModuleBuilder("env").NewFunctionBuilder().
		WithFunc(stateGetBytesInner).Export("stateGetBytes").
		NewFunctionBuilder().WithFunc(stateStoreBytesInner).Export("stateStoreBytes").
		NewFunctionBuilder().WithFunc(stateStoreDynamicBytesInner).Export("stateStoreDynamicBytes").
		NewFunctionBuilder().WithFunc(stateGetDynamicBytesInner).Export("stateGetDynamicBytes").
		Instantiate(ctxWasm)
	if err != nil {
		return nil, nil, err
	}

	_, err = r.NewHostModuleBuilder("precompiles").
		NewFunctionBuilder().WithFunc(gnarkVerify).Export("gnarkVerify").
		NewFunctionBuilder().WithFunc(addBalance).Export("addBalance").
		NewFunctionBuilder().WithFunc(subBalance).Export("subBalance").
		Instantiate(ctxWasm)
	if err != nil {
		return nil, nil, err
	}

	mod, err := r.Instantiate(ctxWasm, wasmByte)
	if err != nil {
		return nil, nil, err
	}

	allocate_ptr = mod.ExportedFunction("allocate_ptr")
	return mod, allocate_ptr, nil
}
