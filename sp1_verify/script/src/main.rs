use sp1_sdk::{ProverClient, SP1Stdin, include_elf};
use std::time::Instant;

fn main() {

    // Setup - accessing the riscv target, getting proving and verifying keys, client and standard input
    let elf = include_elf!("main");
    let client = ProverClient::from_env();
    let (pv, vk) = client.setup(elf);
    let stdin = SP1Stdin::new();

    // This part only exists in sp1, to provide additional info. May be commented out
    println!("===== Execution =====");
    let start_exe = Instant::now();
    let (_, report) = client.execute(elf, &stdin).run().unwrap();
    let exe_time = start_exe.elapsed();
    let prover_gas = report.gas.unwrap_or(0);
    println!("Executed in {:#?}", exe_time);
    println!("Prover gas: {:?}", prover_gas);
    println!("Executed program with {} cycles", report.total_instruction_count());

    println!("Proof generation started!");
    let start = Instant::now();
    let proof = client.prove(&pv, &stdin).run().expect("Failed to prove");
    let prove_time = start.elapsed();
    println!("Proof generated in {:#?}", prove_time);

    let proof_bytes = bincode::serialized_size(&proof).unwrap() as usize;
    println!("Proof size: {}", proof_bytes);

    println!("Verification started!");
    let start = Instant::now();
    client.verify(&proof, &vk).expect("Failed to verify");
    let verify_time = start.elapsed();
    println!("Proof verified in {:#?}", verify_time);

    println!("\n ====== Summary ======");
    println!("Proof generation took: {:#?}", prove_time);
    println!("Verification took: {:#?}", verify_time);
    println!("Proof size: {}", proof_bytes);
}
