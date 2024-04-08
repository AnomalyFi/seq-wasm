package main

import (
	"context"
	_ "embed"
	"errors"
	"fmt"
	"io/ioutil"
	"math/big"
	"os"
	"strconv"

	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// BinaryMerkleProof is an auto generated low-level Go binding around an user-defined struct.
type BinaryMerkleProof struct {
	SideNodes [][32]byte
	Key       *big.Int
	NumLeaves *big.Int
}

// DataRootTuple is an auto generated low-level Go binding around an user-defined struct.
type DataRootTuple struct {
	Height   *big.Int
	DataRoot [32]byte
}

// Signature is an auto generated low-level Go binding around an user-defined struct.
type Signature struct {
	V uint8
	R [32]byte
	S [32]byte
}

// SubmitDataRootTupleRootInput is an auto generated low-level Go binding around an user-defined struct.
type SubmitDataRootTupleRootInput struct {
	NewNonce            *big.Int
	ValidatorSetNonce   *big.Int
	DataRootTupleRoot   [32]byte
	CurrentValidatorSet []Validator
	Sigs                []Signature
}

// UpdateValidatorSetInput is an auto generated low-level Go binding around an user-defined struct.
type UpdateValidatorSetInput struct {
	NewNonce            *big.Int
	OldNonce            *big.Int
	NewPowerThreshold   *big.Int
	NewValidatorSetHash [32]byte
	CurrentValidatorSet []Validator
	Sigs                []Signature
}

// Validator is an auto generated low-level Go binding around an user-defined struct.
type Validator struct {
	Addr  common.Address
	Power *big.Int
}

// VerifyAttestationInput is an auto generated low-level Go binding around an user-defined struct.
type VerifyAttestationInput struct {
	TupleRootNonce *big.Int
	Tuple          DataRootTuple
	Proof          BinaryMerkleProof
}

// InputsInitializerInput is an auto generated low-level Go binding around an user-defined struct.
type InputsInitializerInput struct {
	Nonce                  *big.Int
	PowerThreshold         *big.Int
	ValidatorSetCheckPoint [32]byte
}

// MainMetaData contains all meta data concerning the Main contract.
var MainMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"power_threshold\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"validator_set_check_point\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.InitializerInput\",\"name\":\"_i\",\"type\":\"tuple\"}],\"name\":\"dummyInitializerInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_newNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_validatorSetNonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_dataRootTupleRoot\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"power\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.Validator[]\",\"name\":\"_currentValidatorSet\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.Signature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\"}],\"internalType\":\"structinputs.SubmitDataRootTupleRootInput\",\"name\":\"_s\",\"type\":\"tuple\"}],\"name\":\"dummySubmitDataRootTupleRootInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_newNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_oldNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_newPowerThreshold\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_newValidatorSetHash\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"power\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.Validator[]\",\"name\":\"_currentValidatorSet\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.Signature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\"}],\"internalType\":\"structinputs.UpdateValidatorSetInput\",\"name\":\"_u\",\"type\":\"tuple\"}],\"name\":\"dummyUpdateValidatorSet\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_tupleRootNonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"dataRoot\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.DataRootTuple\",\"name\":\"_tuple\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"sideNodes\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"numLeaves\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.BinaryMerkleProof\",\"name\":\"_proof\",\"type\":\"tuple\"}],\"internalType\":\"structinputs.VerifyAttestationInput\",\"name\":\"_v\",\"type\":\"tuple\"}],\"name\":\"dummyVerifyAttestationInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// MainABI is the input ABI used to generate the binding from.
// Deprecated: Use MainMetaData.ABI instead.
var MainABI, _ = MainMetaData.GetAbi() // modified

func main() {
	wasmByte, _ := ioutil.ReadFile("/home/manojkgorle/nodekit/seq-wasm/blobstream-contracts-rust/target/wasm32-unknown-unknown/release/blobstream_contracts_rust.wasm")
	var allocate_ptr api.Function
	ctxWasm := context.Background()
	r := wazero.NewRuntime(ctxWasm)
	defer r.Close(ctxWasm)
	mapper := map[string][]byte{
		"0": {0},
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
	stateStoreBytesInner := func(ctxInner context.Context, m api.Module, i uint32, ptr uint32, size uint32) {
		slot := "slot" + strconv.Itoa(int(i))
		bytes, ok := m.Memory().Read(ptr, size)
		// fmt.Println(bytes)
		if !ok {
			os.Exit(10)
		}
		mapper[slot] = bytes

	}
	stateGetDynamicBytesInner := func(ctxInner context.Context, m api.Module, offset uint32, key uint32) uint64 {
		i := 128 + (offset*key)%896
		slot := "slot" + strconv.Itoa(int(i))
		result := mapper[slot]
		size := uint64(len(result))
		results, _ := allocate_ptr.Call(ctxInner, size)
		offset2 := results[0]
		m.Memory().Write(uint32(offset2), result)
		return uint64(offset2)<<32 | size
	}
	stateStoreDynamicBytesInner := func(ctxInner context.Context, m api.Module, offset uint32, key uint32, ptr uint32, size uint32) {
		i := 128 + (offset*key)%896
		slot := "slot" + strconv.Itoa(int(i))
		bytes, ok := m.Memory().Read(ptr, size)
		if !ok {
			os.Exit(10)
		}
		mapper[slot] = bytes

	}
	_, err := r.NewHostModuleBuilder("env").NewFunctionBuilder().
		WithFunc(stateGetBytesInner).Export("stateGetBytes").
		NewFunctionBuilder().WithFunc(stateStoreBytesInner).Export("stateStoreBytes").
		NewFunctionBuilder().WithFunc(stateStoreDynamicBytesInner).Export("stateStoreDynamicBytes").
		NewFunctionBuilder().WithFunc(stateGetDynamicBytesInner).Export("stateGetDynamicBytes").
		Instantiate(ctxWasm)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	mod, err := r.Instantiate(ctxWasm, wasmByte)
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	allocate_ptr = mod.ExportedFunction("allocate_ptr")
	deallocate_ptr := mod.ExportedFunction("deallocate_ptr")
	tx_function := mod.ExportedFunction("initializer")
	sdrt_function := mod.ExportedFunction("submit_data_root_tuple_root")
	sdrtri := SubmitDataRootTupleRootInput{
		NewNonce:            big.NewInt(2),
		ValidatorSetNonce:   big.NewInt(1),
		DataRootTupleRoot:   [32]byte(common.Hex2BytesFixed("0de92bac0b356560d821f8e7b6f5c9fe4f3f88f6c822283efd7ab51ad56a640e", 32)),
		CurrentValidatorSet: []Validator{{common.HexToAddress("9c2B12b5a07FC6D719Ed7646e5041A7E85758329"), big.NewInt(5000)}},
		Sigs:                []Signature{{V: 28, R: [32]byte(common.Hex2BytesFixed("f48f949c827fb5a0db3bf416ea657d2750eeadb7b6906c6fb857d2fd1dd57181", 32)), S: [32]byte(common.Hex2BytesFixed("46ae888d1453fd5693b0148cecf0368b42552e597a3b628456946cf63b627b04", 32))}},
	}

	data := InputsInitializerInput{
		Nonce:                  big.NewInt(1),
		PowerThreshold:         big.NewInt(3333),
		ValidatorSetCheckPoint: [32]byte(common.Hex2BytesFixed("4a5cc92ce4a0fb368c83da44ea489e4b908ce75bdc460c31c662f35fd3911ff1", 32)),
	}
	// Encode the parameters using the ABI packer
	packed, err := abi.ABI.Pack(*MainABI, "dummyInitializerInput", data)
	if err != nil {
		fmt.Print(err)
	}
	packed2 := packed[4:] //@todo deal with this, either trim off when sent using relayer or trim of here
	results, err := allocate_ptr.Call(ctxWasm, uint64(len(packed2)))
	if err != nil {
		fmt.Println(err)
		os.Exit(3)
	}
	inputPtr := results[0]

	defer deallocate_ptr.Call(ctxWasm, inputPtr, uint64(len(packed2)))
	mod.Memory().Write(uint32(inputPtr), packed2) // TODO: change
	results, err = tx_function.Call(ctxWasm, inputPtr, uint64(len(packed2)))
	if err != nil {
		fmt.Println(err)
		os.Exit(4)
	}
	fmt.Println(results[0] == 1)

	packed, err = abi.ABI.Pack(*MainABI, "dummySubmitDataRootTupleRootInput", sdrtri)
	if err != nil {
		fmt.Print(err)
	}
	packed2 = packed[4:] //@todo deal with this, either trim off when sent using relayer or trim of here
	results, err = allocate_ptr.Call(ctxWasm, uint64(len(packed2)))
	if err != nil {
		fmt.Println(err)
		os.Exit(3)
	}
	inputPtr = results[0]
	defer deallocate_ptr.Call(ctxWasm, inputPtr, uint64(len(packed2)))
	mod.Memory().Write(uint32(inputPtr), packed2) // TODO: change
	results, err = sdrt_function.Call(ctxWasm, inputPtr, uint64(len(packed2)))
	if err != nil {
		fmt.Println(err)
		os.Exit(4)
	}
	fmt.Println([]byte{byte(results[0])})
}
