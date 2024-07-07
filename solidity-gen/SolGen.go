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

// SolGengnarkPrecompileInputs is an auto generated low-level Go binding around an user-defined struct.
type SolGengnarkPrecompileInputs struct {
	ProgramVKeyHash []byte
	PublicValues    []byte
	ProofBytes      []byte
	ProgramVKey     []byte
}

// SolGenMetaData contains all meta data concerning the SolGen contract.
var SolGenMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"programVKeyHash\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"publicValues\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"proofBytes\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"programVKey\",\"type\":\"bytes\"}],\"internalType\":\"structSolGen.gnarkPrecompileInputs\",\"name\":\"inputs\",\"type\":\"tuple\"}],\"name\":\"gnarkPrecompile\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
}

// SolGenABI is the input ABI used to generate the binding from.
// Deprecated: Use SolGenMetaData.ABI instead.
var SolGenABI = SolGenMetaData.ABI

// SolGen is an auto generated Go binding around an Ethereum contract.
type SolGen struct {
	SolGenCaller     // Read-only binding to the contract
	SolGenTransactor // Write-only binding to the contract
	SolGenFilterer   // Log filterer for contract events
}

// SolGenCaller is an auto generated read-only Go binding around an Ethereum contract.
type SolGenCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SolGenTransactor is an auto generated write-only Go binding around an Ethereum contract.
type SolGenTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SolGenFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type SolGenFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// SolGenSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type SolGenSession struct {
	Contract     *SolGen           // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// SolGenCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type SolGenCallerSession struct {
	Contract *SolGenCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// SolGenTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type SolGenTransactorSession struct {
	Contract     *SolGenTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// SolGenRaw is an auto generated low-level Go binding around an Ethereum contract.
type SolGenRaw struct {
	Contract *SolGen // Generic contract binding to access the raw methods on
}

// SolGenCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type SolGenCallerRaw struct {
	Contract *SolGenCaller // Generic read-only contract binding to access the raw methods on
}

// SolGenTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type SolGenTransactorRaw struct {
	Contract *SolGenTransactor // Generic write-only contract binding to access the raw methods on
}

// NewSolGen creates a new instance of SolGen, bound to a specific deployed contract.
func NewSolGen(address common.Address, backend bind.ContractBackend) (*SolGen, error) {
	contract, err := bindSolGen(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &SolGen{SolGenCaller: SolGenCaller{contract: contract}, SolGenTransactor: SolGenTransactor{contract: contract}, SolGenFilterer: SolGenFilterer{contract: contract}}, nil
}

// NewSolGenCaller creates a new read-only instance of SolGen, bound to a specific deployed contract.
func NewSolGenCaller(address common.Address, caller bind.ContractCaller) (*SolGenCaller, error) {
	contract, err := bindSolGen(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &SolGenCaller{contract: contract}, nil
}

// NewSolGenTransactor creates a new write-only instance of SolGen, bound to a specific deployed contract.
func NewSolGenTransactor(address common.Address, transactor bind.ContractTransactor) (*SolGenTransactor, error) {
	contract, err := bindSolGen(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &SolGenTransactor{contract: contract}, nil
}

// NewSolGenFilterer creates a new log filterer instance of SolGen, bound to a specific deployed contract.
func NewSolGenFilterer(address common.Address, filterer bind.ContractFilterer) (*SolGenFilterer, error) {
	contract, err := bindSolGen(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &SolGenFilterer{contract: contract}, nil
}

// bindSolGen binds a generic wrapper to an already deployed contract.
func bindSolGen(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := SolGenMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SolGen *SolGenRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SolGen.Contract.SolGenCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SolGen *SolGenRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SolGen.Contract.SolGenTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SolGen *SolGenRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SolGen.Contract.SolGenTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_SolGen *SolGenCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _SolGen.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_SolGen *SolGenTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _SolGen.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_SolGen *SolGenTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _SolGen.Contract.contract.Transact(opts, method, params...)
}

// GnarkPrecompile is a paid mutator transaction binding the contract method 0x6d90be9d.
//
// Solidity: function gnarkPrecompile((bytes,bytes,bytes,bytes) inputs) returns(bool)
func (_SolGen *SolGenTransactor) GnarkPrecompile(opts *bind.TransactOpts, inputs SolGengnarkPrecompileInputs) (*types.Transaction, error) {
	return _SolGen.contract.Transact(opts, "gnarkPrecompile", inputs)
}

// GnarkPrecompile is a paid mutator transaction binding the contract method 0x6d90be9d.
//
// Solidity: function gnarkPrecompile((bytes,bytes,bytes,bytes) inputs) returns(bool)
func (_SolGen *SolGenSession) GnarkPrecompile(inputs SolGengnarkPrecompileInputs) (*types.Transaction, error) {
	return _SolGen.Contract.GnarkPrecompile(&_SolGen.TransactOpts, inputs)
}

// GnarkPrecompile is a paid mutator transaction binding the contract method 0x6d90be9d.
//
// Solidity: function gnarkPrecompile((bytes,bytes,bytes,bytes) inputs) returns(bool)
func (_SolGen *SolGenTransactorSession) GnarkPrecompile(inputs SolGengnarkPrecompileInputs) (*types.Transaction, error) {
	return _SolGen.Contract.GnarkPrecompile(&_SolGen.TransactOpts, inputs)
}
