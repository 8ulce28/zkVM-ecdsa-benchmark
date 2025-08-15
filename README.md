# Benchmarking ECDSA Performance across zkVMs

This project benchmarks execution time of a code that verifies ECDSA signature; proof generation time, verification time, and proof size for the execution of that code across [OpenVM](https://github.com/openvm-org/openvm), [SP1](https://github.com/succinctlabs/sp1), [Pico](https://github.com/brevis-network/pico), [Valida](https://github.com/valida-xyz/valida), and [Powdr](https://github.com/powdr-labs/powdr), which are built on/support plonky3. The goal is to provide a fair comparison of the performance characteristics of each system. The `secp256k1` curve was selected as the ECDSA curve since it is one of the only two curves supported by all evaluated zkVMs, the other being `secp256r1`. It is chosen over `secp256r1` because it is shown to be more efficient in zero-knowledge circuits due to its simpler curve equation and broader support in zk tools.

## Setup
Throughout this project, guest code corresponds to the part of the program that runs inside the zkVM and gets proven, while the host code runs outside the VM to compile/access the guest, provide inputs where necessary, generate proof, and verify the proof.

To ensure a fair and maximally optimized benchmark, guest and host codes were tuned separately for each zkVM. To elaborate, sp1 guest code uses a custom-patched version of the k256 crate with removed unnecessary constraints. Openvm uses its own native ecdsa crate. Pico, Valida, powdr use the fastest compatible versions of k256 that work with their execution model and guest environments. Furthermore, chunk sizes and batch sizes were optimized individually for every system.

Further details about the VMs are provided in the README files located in each directory.

## Benchmark Environment
I ran all benchmarks in a MacBook Pro with Apple Silicon chip. It is worth mentioning that Valida does not natively support macOS. Thus, I executed Valida benchmarks inside Docker, which likely introduced non-negligible overhead. This might explain Valida’s performance gap relative to other zkVMs.

## Results
Results regarding one verification per guest code are displayed below.
  
| zkVM    | Proof Generation Time (s) | Verification Time (ms) | Proof Size (bytes) | Execution Time (ms) | 
|---------|:----------------:|:-----------------------:|:-----------:|:----------:|
| SP1     |      11.14       |         763.12          |  9277678    |    55.12   | 
| OpenVM  |    2.517         |           885           |   4173289   |    166     |
| Pico    |       141.17     |     1081.19             |   16436918  |    615     |
| Valida  |       184        |       1344              |   5973657   |    589     |
| Powdr   |      116.31      |            NA           |    11477033 |    740     |

Based on the table, OpenVM offers the fastest proof generation (2.52s) and smallest proof size (app. 4.17 MB). SP1 comes the second in proof generation while it provides the quickest verification. Although the proof generation time in Valida is excessively high (184s), the resulting proof size is comparatively small (app. 5.97MB).

Results for guest codes containing 5 verifications are displayed below. (These are not fully filled as I am still running tests.)

| zkVM    | Proof Generation Time (s) | Verification Time (ms)| Proof Size (bytes) | Execution Time (ms) | 
|---------|:----------------:|:-----------------------:|:-----------:|:----------:|
| SP1     |      30.80       |         922.56          |  12583546   |    253.12  | 
| OpenVM  |    4             |           902           |   4173289   |    270     | 
| Pico    |      TBA         |     TBA                 |   TBA       |    TBA     | -> Runs forever
| Valida  |       1181       |       3684              |   17039410  |    1683    |
| Powdr   |      618.15      |            NA           |    46266660 |    2723.5  |

Results for guest codes containing 10 verifications are displayed below. (These are not filled yet)

| zkVM    | Proof Generation Time (s) | Verification Time (ms)| Proof Size (bytes) | Execution Time (ms) | 
|---------|:----------------:|:-----------------------:|:-----------:|:----------:|
| SP1     |      55.18       |         1518.49         |  19207556   |    495.34  | 
| OpenVM  |    5.16          |          891            |   4173289   |    407     |    
| Pico    |       TBA        |     TBA                 |   TBA       |    TBA     |
| Valida  |       TBA        |      TBA                |   TBA       |   TBA      |
| Powdr   |       NA         |            NA           |    NA       |   5152     | -> Does not run to completion

Powdr host code gets killed while creating program ZK setup.

