package main

import (
	"context"
	"os"

	_ "embed"
	"errors"
	"fmt"
	"math/big"

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

// MainMetaData contains all meta data concerning the Main contract.
var MainMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_newNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_validatorSetNonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_dataRootTupleRoot\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"power\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.Validator[]\",\"name\":\"_currentValidatorSet\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.Signature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\"}],\"internalType\":\"structinputs.SubmitDataRootTupleRootInput\",\"name\":\"_s\",\"type\":\"tuple\"}],\"name\":\"dummySubmitDataRootTupleRootInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_newNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_oldNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_newPowerThreshold\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_newValidatorSetHash\",\"type\":\"bytes32\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"power\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.Validator[]\",\"name\":\"_currentValidatorSet\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.Signature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\"}],\"internalType\":\"structinputs.UpdateValidatorSetInput\",\"name\":\"_u\",\"type\":\"tuple\"}],\"name\":\"dummyUpdateValidatorSet\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_tupleRootNonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"dataRoot\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.DataRootTuple\",\"name\":\"_tuple\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"sideNodes\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"numLeaves\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.BinaryMerkleProof\",\"name\":\"_proof\",\"type\":\"tuple\"}],\"internalType\":\"structinputs.VerifyAttestationInput\",\"name\":\"_v\",\"type\":\"tuple\"}],\"name\":\"dummyVerifyAttestationInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// MainABI is the input ABI used to generate the binding from.
// Deprecated: Use MainMetaData.ABI instead.
var MainABI, _ = MainMetaData.GetAbi() // modified

//go:embed target/wasm32-unknown-unknown/release/input_types_test.wasm
var wasmByte []byte

func main() {
	var allocate_ptr api.Function
	ctxWasm := context.Background()
	r := wazero.NewRuntime(ctxWasm)
	defer r.Close(ctxWasm)

	stateGetBytesInner := func(ctxInner context.Context, m api.Module, i uint32) uint64 {
		bytesa := []byte{0, 2, 3}
		results, _ := allocate_ptr.Call(ctxInner, uint64(len(bytesa)))
		m.Memory().Write(uint32(results[0]), bytesa)
		return uint64(results[0])<<32 | uint64(len(bytesa))
	}
	_, err := r.NewHostModuleBuilder("env").NewFunctionBuilder().WithFunc(stateGetBytesInner).Export("stateGetBytes").Instantiate(ctxWasm)
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
	stdrtr := mod.ExportedFunction("test_get_bytes")

	sdrtri := SubmitDataRootTupleRootInput{
		NewNonce:            big.NewInt(30),
		ValidatorSetNonce:   big.NewInt(489039),
		DataRootTupleRoot:   [32]byte(common.Hex2BytesFixed("82dc1607d84557d3579ce602a45f5872e821c36dbda7ec926dfa17ebc8d5c013", 32)),
		CurrentValidatorSet: []Validator{{common.HexToAddress("9c2B12b5a07FC6D719Ed7646e5041A7E85758329"), big.NewInt(5000)}},
		Sigs:                []Signature{{V: 27, R: [32]byte(common.Hex2BytesFixed("02bd9e5fe41ca09e69c688eb127ba3a710ba0f9f9080b13c1f003126a74be2d5", 32)), S: [32]byte(common.Hex2BytesFixed("6dc6943fc93d17984e3ac3023b15030b33a5c9b6e647ddfb3a7f19a1c3ce9a2e", 32))}},
	}
	// Encode the parameters using the ABI packer
	packed, err := abi.ABI.Pack(*MainABI, "dummySubmitDataRootTupleRootInput", sdrtri)
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
	results, err = stdrtr.Call(ctxWasm /*, inputPtr, uint64(len(packed2))*/)
	if err != nil {
		fmt.Println(err)
		os.Exit(4)
	}
	fmt.Println(results[0] == 10)
}
