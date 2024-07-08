// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package main

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
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

// CommitHeaderRangeInput is an auto generated low-level Go binding around an user-defined struct.
type CommitHeaderRangeInput struct {
	Proof        []byte
	PublicValues []byte
}

// DataRootTuple is an auto generated low-level Go binding around an user-defined struct.
type DataRootTuple struct {
	Height   *big.Int
	DataRoot [32]byte
}

// InitializerInput is an auto generated low-level Go binding around an user-defined struct.
type InitializerInput struct {
	Height                    uint64
	Header                    [32]byte
	BlobstreamProgramVKeyHash []byte
	BlobstreamProgramVKey     []byte
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

// UpdateProgramVkeyInput is an auto generated low-level Go binding around an user-defined struct.
type UpdateProgramVkeyInput struct {
	BlobstreamProgramVKeyHash []byte
	BlobstreamProgramVKey     []byte
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

// BlobStreamInputsABI is the input ABI used to generate the binding from.
// Deprecated: Use BlobStreamInputsMetaData.ABI instead.
var BlobStreamInputsABI = BlobStreamInputsMetaData.ABI

// BlobStreamInputs is an auto generated Go binding around an Ethereum contract.
type BlobStreamInputs struct {
	BlobStreamInputsCaller     // Read-only binding to the contract
	BlobStreamInputsTransactor // Write-only binding to the contract
	BlobStreamInputsFilterer   // Log filterer for contract events
}

// BlobStreamInputsCaller is an auto generated read-only Go binding around an Ethereum contract.
type BlobStreamInputsCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// BlobStreamInputsTransactor is an auto generated write-only Go binding around an Ethereum contract.
type BlobStreamInputsTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// BlobStreamInputsFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type BlobStreamInputsFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// BlobStreamInputsSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type BlobStreamInputsSession struct {
	Contract     *BlobStreamInputs // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// BlobStreamInputsCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type BlobStreamInputsCallerSession struct {
	Contract *BlobStreamInputsCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts           // Call options to use throughout this session
}

// BlobStreamInputsTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type BlobStreamInputsTransactorSession struct {
	Contract     *BlobStreamInputsTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts           // Transaction auth options to use throughout this session
}

// BlobStreamInputsRaw is an auto generated low-level Go binding around an Ethereum contract.
type BlobStreamInputsRaw struct {
	Contract *BlobStreamInputs // Generic contract binding to access the raw methods on
}

// BlobStreamInputsCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type BlobStreamInputsCallerRaw struct {
	Contract *BlobStreamInputsCaller // Generic read-only contract binding to access the raw methods on
}

// BlobStreamInputsTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type BlobStreamInputsTransactorRaw struct {
	Contract *BlobStreamInputsTransactor // Generic write-only contract binding to access the raw methods on
}

// NewBlobStreamInputs creates a new instance of BlobStreamInputs, bound to a specific deployed contract.
func NewBlobStreamInputs(address common.Address, backend bind.ContractBackend) (*BlobStreamInputs, error) {
	contract, err := bindBlobStreamInputs(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &BlobStreamInputs{BlobStreamInputsCaller: BlobStreamInputsCaller{contract: contract}, BlobStreamInputsTransactor: BlobStreamInputsTransactor{contract: contract}, BlobStreamInputsFilterer: BlobStreamInputsFilterer{contract: contract}}, nil
}

// NewBlobStreamInputsCaller creates a new read-only instance of BlobStreamInputs, bound to a specific deployed contract.
func NewBlobStreamInputsCaller(address common.Address, caller bind.ContractCaller) (*BlobStreamInputsCaller, error) {
	contract, err := bindBlobStreamInputs(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &BlobStreamInputsCaller{contract: contract}, nil
}

// NewBlobStreamInputsTransactor creates a new write-only instance of BlobStreamInputs, bound to a specific deployed contract.
func NewBlobStreamInputsTransactor(address common.Address, transactor bind.ContractTransactor) (*BlobStreamInputsTransactor, error) {
	contract, err := bindBlobStreamInputs(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &BlobStreamInputsTransactor{contract: contract}, nil
}

// NewBlobStreamInputsFilterer creates a new log filterer instance of BlobStreamInputs, bound to a specific deployed contract.
func NewBlobStreamInputsFilterer(address common.Address, filterer bind.ContractFilterer) (*BlobStreamInputsFilterer, error) {
	contract, err := bindBlobStreamInputs(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &BlobStreamInputsFilterer{contract: contract}, nil
}

// bindBlobStreamInputs binds a generic wrapper to an already deployed contract.
func bindBlobStreamInputs(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := BlobStreamInputsMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_BlobStreamInputs *BlobStreamInputsRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _BlobStreamInputs.Contract.BlobStreamInputsCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_BlobStreamInputs *BlobStreamInputsRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.BlobStreamInputsTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_BlobStreamInputs *BlobStreamInputsRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.BlobStreamInputsTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_BlobStreamInputs *BlobStreamInputsCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _BlobStreamInputs.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_BlobStreamInputs *BlobStreamInputsTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_BlobStreamInputs *BlobStreamInputsTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.contract.Transact(opts, method, params...)
}

// CommitHeaderRange is a paid mutator transaction binding the contract method 0x4710c5f5.
//
// Solidity: function commitHeaderRange((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) CommitHeaderRange(opts *bind.TransactOpts, inputs CommitHeaderRangeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "commitHeaderRange", inputs)
}

// CommitHeaderRange is a paid mutator transaction binding the contract method 0x4710c5f5.
//
// Solidity: function commitHeaderRange((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) CommitHeaderRange(inputs CommitHeaderRangeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.CommitHeaderRange(&_BlobStreamInputs.TransactOpts, inputs)
}

// CommitHeaderRange is a paid mutator transaction binding the contract method 0x4710c5f5.
//
// Solidity: function commitHeaderRange((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) CommitHeaderRange(inputs CommitHeaderRangeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.CommitHeaderRange(&_BlobStreamInputs.TransactOpts, inputs)
}

// Initializer is a paid mutator transaction binding the contract method 0xff231ada.
//
// Solidity: function initializer((uint64,bytes32,bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) Initializer(opts *bind.TransactOpts, inputs InitializerInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "initializer", inputs)
}

// Initializer is a paid mutator transaction binding the contract method 0xff231ada.
//
// Solidity: function initializer((uint64,bytes32,bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) Initializer(inputs InitializerInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.Initializer(&_BlobStreamInputs.TransactOpts, inputs)
}

// Initializer is a paid mutator transaction binding the contract method 0xff231ada.
//
// Solidity: function initializer((uint64,bytes32,bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) Initializer(inputs InitializerInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.Initializer(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateFreeze is a paid mutator transaction binding the contract method 0x173bb932.
//
// Solidity: function updateFreeze((bool) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) UpdateFreeze(opts *bind.TransactOpts, inputs UpdateFreezeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "updateFreeze", inputs)
}

// UpdateFreeze is a paid mutator transaction binding the contract method 0x173bb932.
//
// Solidity: function updateFreeze((bool) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) UpdateFreeze(inputs UpdateFreezeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateFreeze(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateFreeze is a paid mutator transaction binding the contract method 0x173bb932.
//
// Solidity: function updateFreeze((bool) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) UpdateFreeze(inputs UpdateFreezeInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateFreeze(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateGenesisState is a paid mutator transaction binding the contract method 0x541466cf.
//
// Solidity: function updateGenesisState((uint64,bytes32) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) UpdateGenesisState(opts *bind.TransactOpts, inputs UpdateGenesisStateInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "updateGenesisState", inputs)
}

// UpdateGenesisState is a paid mutator transaction binding the contract method 0x541466cf.
//
// Solidity: function updateGenesisState((uint64,bytes32) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) UpdateGenesisState(inputs UpdateGenesisStateInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateGenesisState(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateGenesisState is a paid mutator transaction binding the contract method 0x541466cf.
//
// Solidity: function updateGenesisState((uint64,bytes32) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) UpdateGenesisState(inputs UpdateGenesisStateInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateGenesisState(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateProgramVkey is a paid mutator transaction binding the contract method 0xf3ca5c8d.
//
// Solidity: function updateProgramVkey((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) UpdateProgramVkey(opts *bind.TransactOpts, inputs UpdateProgramVkeyInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "updateProgramVkey", inputs)
}

// UpdateProgramVkey is a paid mutator transaction binding the contract method 0xf3ca5c8d.
//
// Solidity: function updateProgramVkey((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) UpdateProgramVkey(inputs UpdateProgramVkeyInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateProgramVkey(&_BlobStreamInputs.TransactOpts, inputs)
}

// UpdateProgramVkey is a paid mutator transaction binding the contract method 0xf3ca5c8d.
//
// Solidity: function updateProgramVkey((bytes,bytes) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) UpdateProgramVkey(inputs UpdateProgramVkeyInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.UpdateProgramVkey(&_BlobStreamInputs.TransactOpts, inputs)
}

// VerifyAppend is a paid mutator transaction binding the contract method 0x2122bc38.
//
// Solidity: function verifyAppend((uint256,(uint256,bytes32),(bytes32[],uint256,uint256)) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactor) VerifyAppend(opts *bind.TransactOpts, inputs VAInput) (*types.Transaction, error) {
	return _BlobStreamInputs.contract.Transact(opts, "verifyAppend", inputs)
}

// VerifyAppend is a paid mutator transaction binding the contract method 0x2122bc38.
//
// Solidity: function verifyAppend((uint256,(uint256,bytes32),(bytes32[],uint256,uint256)) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsSession) VerifyAppend(inputs VAInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.VerifyAppend(&_BlobStreamInputs.TransactOpts, inputs)
}

// VerifyAppend is a paid mutator transaction binding the contract method 0x2122bc38.
//
// Solidity: function verifyAppend((uint256,(uint256,bytes32),(bytes32[],uint256,uint256)) inputs) returns()
func (_BlobStreamInputs *BlobStreamInputsTransactorSession) VerifyAppend(inputs VAInput) (*types.Transaction, error) {
	return _BlobStreamInputs.Contract.VerifyAppend(&_BlobStreamInputs.TransactOpts, inputs)
}
