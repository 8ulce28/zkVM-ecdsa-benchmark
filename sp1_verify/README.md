# SP1 Benchmarking Code

This directory contains the guest code -inside `program` directory- and the host code -inside `script` directory- that proves and verifies 
the execution of the guest code. To install SP1 zkVM, please refer to the [official SP1 tutorial](https://docs.succinct.xyz/docs/sp1/getting-started/install).

## Running the Code

The program needs to be compiled to a RISC-V file using the command `cargo prove build` to be proven in any host code. During compilation, SP1 sets an environment variable in the form of `SP1_ELF_main=/path`, which points to the path of the compiled ELF (short for Executable and Linkable Format) binary. To make this path accessible to the host code, the environment variable needs to be exported to the top-level scope so that the host code can reference it while loading the ELF. Running `export SP1_ELF_main=/path` is therefore crucial.
