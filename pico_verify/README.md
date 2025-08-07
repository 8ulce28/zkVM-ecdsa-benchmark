# Pico Benchmarking Code

This directory contains the guest code (inside `app`) and the host code (inside `prover`) that proves and verifies the execution of 
the guest code. For installing Pico, please refer to the [official Pico tutorial](https://pico-docs.brevis.network/getting-started/installation).

## Running the Code

As the first step, the guest code needs to be compiled to an ELF file similarly to other VMs. Therefore, we run `cargo pico build` in `app` directory.
Then, we build the `prover` by running `cargo build --release`. Finally, to display the results,
```
CHUNK_SIZE=262144 CHUNK_BATCH_SIZE=2 cargo run --release
```
should be run where `CHUNK_SIZE` specifies the number of cycles the VM executes before dividing the computation into chunks and can be set to any 
power of 2; and where `CHUNK_BATCH_SIZE` indicates how many chunks are processed at the same time and can be any number sufficiently small to prevent 
the program from running out of memory. After experimenting with these two variables, I decided to set `CHUNK_SIZE=524288 = 2^19`, `CHUNK_BATCH_SIZE=1`
as the table shows:

| CHUNK_SIZE | CHUNK_BATCH_SIZE | Proof Generation Time (s) |
|------------------|------------|----------------|
| 2^18 (262,144)    | 1          | pointer being freed was not allocated         |
| 2^18 (262,144)   | 2          | 295.5 s            |
| 2^18 (262,144)   | 8         | >7 mins        |
| 2^20 (1,048,576) | 4          | 5.1            |
