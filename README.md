# Benchmarking ECDSA Performance across zkVMs

This project benchmarks execution time of a code that verifies ECDSA signature; proof generation time, verification time, and proof size for the execution of that code across several zero-knowledge virtual machines (zkVMs) that are built on plonky3. The goal is to provide a fair comparison of the performance characteristics of each system. The `secp256k1` curve was selected as the ECDSA curve since it is one of the only two curves supported by all evaluated zkVMs, the other being `secp256r1`. It is chosen over `secp256r1` because it is shown to be more efficient in zero-knowledge circuits due to its simpler curve equation and broader support in zk tools.

## Setup
Throughout this project, guest code corresponds to the part of the program that runs inside the zkVM and gets proven, while the host code runs outside the VM to compile/access the guest, provide inputs where necessary, generate proof, and verify the proof.

To ensure a fair and maximally optimized benchmark, guest and host codes were tuned separately for each zkVM. To elaborate, sp1 guest code uses a custom-patched version of the k256 crate with removed unnecessary constraints. Openvm uses its own native ecdsa crate. Pico, Valida, powdr use the fastest compatible versions of k256 that work with their execution model and guest environments. Furthermore, chunk sizes and batch sizes were optimized individually for every system.

## Benchmark Environment
I ran all benchmarks in a MacBook Pro with Apple Silicon chip. Regarding this, it is worth mentioning that Valida does not natively support macOS. Thus, I executed Valida benchmarks inside Docker, which likely introduced non-negligible overhead. This might explain Valida’s performance gap relative to other zkVMs.
