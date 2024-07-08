package main

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"math/big"
	"os"
	"strconv"
	"strings"
	"unsafe"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/frontend"
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

// CommitHeaderRangeInput is an auto generated low-level Go binding around an user-defined struct.
type CommitHeaderRangeInput struct {
	Proof        []byte
	PublicValues []byte
}

// UpdateFreezeInput is an auto generated low-level Go binding around an user-defined struct.
type UpdateFreezeInput struct {
	Freeze bool
}

// UpdateGenesisStateInput is an auto generated low-level Go binding around an user-defined struct.
type UpdateGenesisStateInput struct {
	Height uint64
	Header [32]byte
}

// InitializerInput is an auto generated low-level Go binding around an user-defined struct.
type InitializerInput struct {
	Height                    uint64
	Header                    [32]byte
	BlobstreamProgramVKeyHash []byte
	BlobstreamProgramVKey     []byte
}

// UpdateProgramVkeyInput is an auto generated low-level Go binding around an user-defined struct.
type UpdateProgramVkeyInput struct {
	BlobstreamProgramVKeyHash []byte
	BlobstreamProgramVKey     []byte
}

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

// VAInput is an auto generated low-level Go binding around an user-defined struct.
type VAInput struct {
	TupleRootNonce *big.Int
	Tuple          DataRootTuple
	Proof          BinaryMerkleProof
}

// BlobStreamInputsMetaData contains all meta data concerning the BlobStreamInputs contract.
var BlobStreamInputsMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"publicValues\",\"type\":\"bytes\"}],\"internalType\":\"structCommitHeaderRangeInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"commitHeaderRange\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint64\",\"name\":\"height\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"header\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"blobstreamProgramVKeyHash\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"blobstreamProgramVKey\",\"type\":\"bytes\"}],\"internalType\":\"structInitializerInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"initializer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bool\",\"name\":\"freeze\",\"type\":\"bool\"}],\"internalType\":\"structUpdateFreezeInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"updateFreeze\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint64\",\"name\":\"height\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"header\",\"type\":\"bytes32\"}],\"internalType\":\"structUpdateGenesisStateInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"updateGenesisState\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"blobstreamProgramVKeyHash\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"blobstreamProgramVKey\",\"type\":\"bytes\"}],\"internalType\":\"structUpdateProgramVkeyInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"updateProgramVkey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"tuple_root_nonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"dataRoot\",\"type\":\"bytes32\"}],\"internalType\":\"structDataRootTuple\",\"name\":\"tuple\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"sideNodes\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"numLeaves\",\"type\":\"uint256\"}],\"internalType\":\"structBinaryMerkleProof\",\"name\":\"proof\",\"type\":\"tuple\"}],\"internalType\":\"structVAInput\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"verifyAppend\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

var BlobStreamInputsABI, _ = BlobStreamInputsMetaData.GetAbi()

type TxContext struct {
	timestamp    int64
	msgSenderPtr uint32
}

func txContextToBytes(c TxContext) []byte {
	// creates array of length 2^10 and access the memory at struct c to have enough space for all the struct.
	// [:size:size] slices array to size and fixes array size as size
	size := unsafe.Sizeof(c)
	bytes := (*[1 << 10]byte)(unsafe.Pointer(&c))[:size:size]
	return bytes
}

type GnarkPrecompileInputs struct {
	ProgramVKeyHash []uint8 `json:"programVKeyHash"`
	PublicValues    []uint8 `json:"publicValues"`
	ProofBytes      []uint8 `json:"proofBytes"`
	ProgramVKey     []uint8 `json:"programVKey"`
}

// GnarkPreCompileMetaData contains all meta data concerning the SolGen contract.
var GnarkPreCompileMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"programVKeyHash\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"publicValues\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"proofBytes\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"programVKey\",\"type\":\"bytes\"}],\"internalType\":\"structSolGen.gnarkPrecompileInputs\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"gnarkPrecompile\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

var GnarkPreCompileABI, _ = GnarkPreCompileMetaData.GetAbi()

var mask = new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), 253), big.NewInt(1))

type SP1Circuit struct {
	VkeyHash             frontend.Variable `gnark:",public"`
	CommitedValuesDigest frontend.Variable `gnark:",public"`
	Vars                 []frontend.Variable
	Felts                []babybearVariable
	Exts                 []babybearExtensionVariable
}

func (*SP1Circuit) Define(frontend.API) error {
	return nil
}

type babybearVariable struct {
	Value  frontend.Variable
	NbBits uint
}

type babybearExtensionVariable struct {
	Value [4]babybearVariable
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
		// read from memory
		dataBytes, ok := m.Memory().Read(ptr, size)
		if !ok {
			return 0
		}
		// abi unpack the data
		method := GnarkPreCompileABI.Methods["gnarkPrecompile"]
		upack, err := method.Inputs.Unpack(dataBytes)
		if err != nil {
			return 0
		}
		preCompileInput := upack[0].(struct {
			ProgramVKeyHash []byte `json:"programVKeyHash"`
			PublicValues    []byte `json:"publicValues"`
			ProofBytes      []byte `json:"proofBytes"`
			ProgramVKey     []byte `json:"programVKey"`
		})
		publicValuesHash := sha256.Sum256(preCompileInput.PublicValues)
		publicValuesB := new(big.Int).SetBytes(publicValuesHash[:])
		publicValuesDigest := new(big.Int).And(publicValuesB, mask)
		if publicValuesDigest.BitLen() > 253 {
			return 0
		}

		sp1Circuit := SP1Circuit{
			Vars:                 []frontend.Variable{},
			Felts:                []babybearVariable{},
			Exts:                 []babybearExtensionVariable{},
			VkeyHash:             string(preCompileInput.ProgramVKeyHash),
			CommitedValuesDigest: publicValuesDigest,
		}

		// fmt.Println(sp1Circuit.VkeyHash)
		// read vk from preCompileInput
		vk := plonk.NewVerifyingKey(ecc.BN254)
		_, err = vk.ReadFrom(bytes.NewBuffer(preCompileInput.ProgramVKey))
		if err != nil {
			return 0
		}

		// read proof from preCompileInput
		proof := plonk.NewProof(ecc.BN254)
		proofData, err := hex.DecodeString(string(preCompileInput.ProofBytes))
		if err != nil {
			return 0
		}
		_, err = proof.ReadFrom(bytes.NewReader(proofData))
		if err != nil {
			return 0
		}

		// create witness
		wit, err := frontend.NewWitness(&sp1Circuit, ecc.BN254.ScalarField())
		if err != nil {
			fmt.Println(err)
			return 0
		}

		// get the public witness
		pubWit, err := wit.Public()
		if err != nil {
			fmt.Println(err)
			return 0
		}

		// verify the proof
		err = plonk.Verify(proof, vk, pubWit)
		if err != nil {
			fmt.Println(err)
			// the vk may not be corresponding to the proof or public witness are not corresponding to proofs or proof is invalid
			return 0
		}
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
