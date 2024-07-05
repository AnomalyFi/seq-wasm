# sol gen tools

```bash
solc --abi gen.sol -o build
abigen --abi build/SolGen.abi --pkg main --type SolGen --out SolGen.go
```