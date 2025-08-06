# Valida Benchmarking Code

This directory contains the guest code (inside `valid_a_verify/guest`), the host code that might be used to keep track of the proof generation 
and verification time  (inside `valid_a_verify/host`), and an additional helper file (inside `measure`) that extracts the proof size. For installing 
Valida VM, please refer to the [official Valida tutorial](https://lita.gitbook.io/lita-documentation/quick-start/installation-and-system-requirements).
As written in the link, Valida currently does not natively support ARM64 architectures. Thus, in such architectures, one must use Docker to build and 
run valida in an x86_64 emulated environment. 

## Running the Code
