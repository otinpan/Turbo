# Turbo
## Build
```
cmake -S . -B build
cmake --build build
```
If build on WSL, GPU performance decline because of emulation.
```
Found 1 devices
Device 0:
  Name: llvmpipe (LLVM 20.1.2, 256 bits)
  API Version: 1.4.318
  Vendor ID: 65541
  Device ID: 0
  Type: Cpu

Queue family 0: { Graphics | Compute | Transfer | SparseBinding }
```