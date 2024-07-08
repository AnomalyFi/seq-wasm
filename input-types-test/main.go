package main

import (
	"bytes"
	"context"
	"crypto/sha256"
	_ "embed"
	"encoding/hex"
	"errors"
	"fmt"
	"io/ioutil"
	"math/big"
	"os"
	"strconv"
	"strings"

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

// BinaryMerkleProof is an auto generated low-level Go binding around an user-defined struct.
type BinaryMerkleProof struct {
	SideNodes [][32]byte
	Key       *big.Int
	NumLeaves *big.Int
}

// // CommitHeaderRangeInput is an auto generated low-level Go binding around an user-defined struct.
// type CommitHeaderRangeInput struct {
// 	TargetBlock uint64
// 	Input       []byte
// 	Output      []byte
// 	Proof       []byte
// }

// DataRootTuple is an auto generated low-level Go binding around an user-defined struct.
type DataRootTuple struct {
	Height   *big.Int
	DataRoot [32]byte
}

// InitializerInput is an auto generated low-level Go binding around an user-defined struct.
// type InitializerInput struct {
// 	Height uint64
// 	Header [32]byte
// }

// OutputBreaker is an auto generated low-level Go binding around an user-defined struct.
type OutputBreaker struct {
	TargetHeader   [32]byte
	DataCommitment [32]byte
}

// VerifyAttestationInput is an auto generated low-level Go binding around an user-defined struct.
type VerifyAttestationInput struct {
	TupleRootNonce *big.Int
	Tuple          DataRootTuple
	Proof          BinaryMerkleProof
}

// MainMetaData contains all meta data concerning the Main contract.
var MainMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"uint64\",\"name\":\"targetBlock\",\"type\":\"uint64\"},{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"}],\"internalType\":\"structinputs.CommitHeaderRangeInput\",\"name\":\"_c\",\"type\":\"tuple\"}],\"name\":\"dummyCommitHeaderRangeInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint64\",\"name\":\"height\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"header\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.InitializerInput\",\"name\":\"_i\",\"type\":\"tuple\"}],\"name\":\"dummyInitializerInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes32\",\"name\":\"targetHeader\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"dataCommitment\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.OutputBreaker\",\"name\":\"_o\",\"type\":\"tuple\"}],\"name\":\"dummyOutputBreaker\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"_tupleRootNonce\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"height\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"dataRoot\",\"type\":\"bytes32\"}],\"internalType\":\"structinputs.DataRootTuple\",\"name\":\"_tuple\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes32[]\",\"name\":\"sideNodes\",\"type\":\"bytes32[]\"},{\"internalType\":\"uint256\",\"name\":\"key\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"numLeaves\",\"type\":\"uint256\"}],\"internalType\":\"structinputs.BinaryMerkleProof\",\"name\":\"_proof\",\"type\":\"tuple\"}],\"internalType\":\"structinputs.VerifyAttestationInput\",\"name\":\"_v\",\"type\":\"tuple\"}],\"name\":\"dummyVerifyAttestationInput\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// SolGengnarkPrecompileInputs is an auto generated low-level Go binding around an user-defined struct.
// type GnarkPrecompileInputs struct {
// 	ProgramVKeyHash []uint8 `json:"programVKeyHash"`
// 	PublicValues    []uint8 `json:"publicValues"`
// 	ProofBytes      []uint8 `json:"proofBytes"`
// 	ProgramVKey     []uint8 `json:"programVKey"`
// }

// GnarkPreCompileMetaData contains all meta data concerning the SolGen contract.
var GnarkPreCompileMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"programVKeyHash\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"publicValues\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"proofBytes\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"programVKey\",\"type\":\"bytes\"}],\"internalType\":\"structSolGen.gnarkPrecompileInputs\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"gnarkPrecompile\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// MainABI is the input ABI used to generate the binding from.
// Deprecated: Use MainMetaData.ABI instead.
var MainABI, _ = MainMetaData.GetAbi() // modified
// var GnarkABI, _ = GnarkPrecompMetaData.GetAbi()
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

func main() {
	wasmByte, _ := ioutil.ReadFile("/home/manojkgorle/nodekit/seq-wasm/blobstream-contracts-rust/target/wasm32-unknown-unknown/release/blobstream_contracts_rust.wasm")
	var allocate_ptr api.Function
	ctxWasm := context.Background()
	r := wazero.NewRuntime(ctxWasm)
	defer r.Close(ctxWasm)
	mapper := map[string][]byte{
		"0": {0},
	}
	chri := CommitHeaderRangeInput{}
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
	gnarkVer := func(ctxInner context.Context, m api.Module, ptr uint32, size uint32) uint32 {
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
		preCompileInput := upack[0].(*GnarkPrecompileInputs)
		publicValuesHash := sha256.Sum256(preCompileInput.PublicValues)
		publicValuesB := new(big.Int).SetBytes(publicValuesHash[:])
		publicValuesM := new(big.Int).And(publicValuesB, mask)
		if publicValuesM.BitLen() > 253 {
			return 0
		}
		sp1Circuit := SP1Circuit{
			Vars:                 []frontend.Variable{},
			Felts:                []babybearVariable{},
			Exts:                 []babybearExtensionVariable{},
			VkeyHash:             preCompileInput.ProgramVKey,
			CommitedValuesDigest: publicValuesM,
		}

		vk := plonk.NewVerifyingKey(ecc.BN254)
		_, err = vk.ReadFrom(bytes.NewBuffer(preCompileInput.ProgramVKey))
		if err != nil {
			fmt.Printf("failed to read vk file: %s", err)
			return 0
		}

		proof := plonk.NewProof(ecc.BN254)
		proofData, err := hex.DecodeString(string(preCompileInput.ProofBytes))
		if err != nil {
			panic(err)
		}
		_, err = proof.ReadFrom(bytes.NewReader(proofData))
		if err != nil {
			fmt.Println(err)
			return 0
		}
		wit, _ := frontend.NewWitness(&sp1Circuit, ecc.BN254.ScalarField())
		pubWit, _ := wit.Public()
		err = plonk.Verify(proof, vk, pubWit)
		if err != nil {
			return 0
		}
		return 1
	}

	// Instantiate the module
	_, err := r.NewHostModuleBuilder("env").NewFunctionBuilder().
		WithFunc(stateGetBytesInner).Export("stateGetBytes").
		NewFunctionBuilder().WithFunc(stateStoreBytesInner).Export("stateStoreBytes").
		NewFunctionBuilder().WithFunc(stateStoreDynamicBytesInner).Export("stateStoreDynamicBytes").
		NewFunctionBuilder().WithFunc(stateGetDynamicBytesInner).Export("stateGetDynamicBytes").
		NewFunctionBuilder().WithFunc(gnarkVer).Export("gnarkVerify").
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
	chr_function := mod.ExportedFunction("commit_header_range")

	data := InitializerInput{
		Height: 1,
		Header: [32]byte(common.Hex2BytesFixed("4a5cc92ce4a0fb368c83da44ea489e4b908ce75bdc460c31c662f35fd3911ff1", 32)),
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

	packed, err = abi.ABI.Pack(*MainABI, "dummyCommitHeaderRangeInput", chri)
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
	results, err = chr_function.Call(ctxWasm, inputPtr, uint64(len(packed2)))
	if err != nil {
		fmt.Println(err)
		os.Exit(4)
	}
	fmt.Println([]byte{byte(results[0])})
}
