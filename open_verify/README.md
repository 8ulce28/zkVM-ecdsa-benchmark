# OpenVM Benchmarking Code

This directory contains the guest code (inside `guest`), the host code that might be used to keep track of the proof generation 
and verification time (inside `host`), and a helper file (inside` measure`) that extracts the proof size. To install openVM, please refer to the
[official OpenVM tutorial](https://book.openvm.dev/getting-started/install.html).

## Running the Code
The CLI is used in this part of benchmarking because the host code failed due to a parse error during guest-to-VM executable compilation. The error occurs despite the code matching the structure of the official examples, and the GitHub reference. This might be due to insufficient/outdated documentation. The designed host code rests in `host` for further checks.

All the following codes are run inside `guest` directory.

To build the guest, please use `cargo openvm build`. After building, 
```
time cargo openvm run --exe target/openvm/release/guest.vmexe
```
is used to time the execution of the guest code. Then, we create proving and verifying keys by running `cargo openvm keygen`. To time the proof 
generation in application level,
```
time cargo openvm prove app --exe target/openvm/release/guest.vmexe
```
; and to time the verification of the proof,
```
time cargo openvm verify app --proof guest.app.proof
```
commands can be used. Running `measure` displays the proof size as the final step.
