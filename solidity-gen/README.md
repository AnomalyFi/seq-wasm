# sol gen tools

```bash
solc --abi gen.sol -o build
abigen --abi build/SolGen.abi --pkg main --type SolGen --out SolGen.go

solc --abi blobstream.sol -o build --overwrite
abigen --abi build/BlobStreamInputs.abi --pkg main --type BlobStreamInputs --out BlobStreamInputs.go
```