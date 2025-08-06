# powdrVM Benchmarking Code

This directory contains the guest code, inside `guest` directory, to be proven and the main code for benchmarking proof generation time and proof size.
For installation of powdr, please refer to the [official powdr tutorial](https://docs.powdr.org/installation_sdk.html).

## Running the Host Code
Unlike other VMs, powdr does not require a separate compilation of the guest code. Running `cargo run -r` from the root is sufficient for 
the host to prove the execution of the guest. Upon running this command, compiled pil constraints, asm, and all information - constants, commits, proofs - regarding chunks will be located in a directory called `powdr-target`. We access this directory to extract the total proof size.

powdr performs a setup process to generate the proving and verifying keys before generation the proof for the guest program, and
this process may take some time during the first run. However, later runs are quicker as the setup only needs to be redone iff 
the guest crate changes. Furthermore, TO CONTINUE
