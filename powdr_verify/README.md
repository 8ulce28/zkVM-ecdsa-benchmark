# powdrVM Benchmarking Code

This directory contains the guest code, inside `guest` directory, to be proven and the main code for benchmarking proof generation time and proof size.
For installation of powdr, please refer to the [official powdr tutorial](https://docs.powdr.org/installation_sdk.html).

## Running the Host Code
Unlike other VMs, powdr does not require a separate compilation of the guest code. Running `cargo run -r` from the root is sufficient for 
the host to prove the execution of the guest. Upon running this command, compiled pil constraints, asm, and all information - constants, commits, 
proofs - regarding chunks will be located in a directory called `powdr-target`. We access this directory to extract the total proof size in the host.

powdr performs a setup process to generate the proving and verifying keys before generation the proof for the guest program, and
this process may take some time during the first run. However, later runs are quicker as the setup only needs to be redone iff 
the guest crate changes. Also, powdr splits long traces into chunks that are each proven independently. The default chunk size is **2^20 (1,048,576 
rows)**. After experimenting with the values **2^18, 2^19**, and **2^20**, I decided to set the size equal to **2^20** based on proof generation time and proof size as shown in the table:

| Chunk Size (2^n) | Proof Generation Time (s) | Proof Size (bytes) |
|------------------|----------------|-------------|      
| 2^18 (262144)    | 120.28 s      | 42242980 |
| 2^19 (524288)   |  119.07 s      | 22020178 |
| 2^20 (1048576) |  120.12 s       | 11477033 |


While evaluating the performance, there occurred some issues when attempting to verify generated proofs: powdr generates one proof per chunk during 
proving. However, the powdr verify CLI expects a single proof file, not a list or folder of chunked proofs. Therefore, CLI becomes non-trivial unless 
manual aggregation is done which is undocumented. Moreover, there is no documentation on or working example showing how to load and verify proofs from 
the host. Thus, all attempts to verify proofs programmatically failed.
