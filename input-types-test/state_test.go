package main

import (
	"context"
	"encoding/hex"
	"os"
	"strconv"
	"testing"

	"github.com/AnomalyFi/hypersdk/codec"
	"github.com/stretchr/testify/require"
)

func TestState(t *testing.T) {
	wasmByte, _ := os.ReadFile("../target/wasm32-unknown-unknown/release/input_types_test.wasm")

	ctxWasm := context.Background()
	mapper := map[string][]byte{
		"0": {0},
	}

	mod, allocate_ptr, err := runtime(ctxWasm, mapper, wasmByte)
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
	test_multi_input := mod.ExportedFunction("test_multi_input")
	test_tx_context := mod.ExportedFunction("test_tx_context")

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

	result, err = test_multi_input.Call(ctxWasm, 1, 2, 4, 5)
	require.NoError(t, err)
	require.Equal(t, result[0], uint64(12))

	// test tx context
	// Allocate and write to memory message sender and tx context.
	results, err := allocate_ptr.Call(ctxWasm, codec.AddressLen)
	require.NoError(t, err)
	address_ptr := uint32(results[0])
	actor := []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33}
	timeStamp := int64(149)
	mod.Memory().Write(address_ptr, actor[:])

	txContext := TxContext{timestamp: timeStamp, msgSenderPtr: address_ptr}
	txContextBytes := txContextToBytes(txContext)

	txContextPtr := address_ptr + 33
	mod.Memory().Write(uint32(txContextPtr), txContextBytes)

	results, err = test_tx_context.Call(ctxWasm, uint64(txContextPtr))
	require.NoError(t, err)
	require.Equal(t, uint64(1), results[0])
}
