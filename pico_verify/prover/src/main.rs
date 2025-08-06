use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;

fn main() {

    // Setting up the client
    init_logger();

    let elf = include_bytes!("../../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(elf);

    let mut stdin = client.new_stdin_builder();

    // This is dummy input. Pico insists on having input written in stdin
    stdin.write(&1);
    let new_stdin = stdin.clone();

    println!("Proof generation started!");
    let begin = Instant::now();
    let f_proof = client.prove_fast(new_stdin).expect("Failed to prove fast :(");
    let finish = begin.elapsed();
    println!("Proof generation took {:#?}", finish);

    // This part is run in order to get the proving key only. The proofs generated above and here are the same.
    println!("Proof generation to get the proving key started!");
    let begin = Instant::now();
    let (proof, pv) = client.prove(stdin).expect("Failed to prove");
    let finish2 = begin.elapsed();
    println!("It took {:#?}", finish2); // For proof gen. time benchmarking, use prove_fast()

    let proof_bytes = bincode::serialized_size(&f_proof.proofs).unwrap() as usize;
    println!("Proof size: {} bytes", proof_bytes);

    let begin = Instant::now();
    let _verified = client.verify(&(proof, pv)).expect("Failed to verify");
    let finish_verify = begin.elapsed();
    println!("Verification took {:#?}", finish_verify);

    println!("\n====== Summary ======");
    println!("Proof generation took: {:#?}", finish);
    println!("Verification took: {:#?}", finish_verify);
    println!("Proof size: {} bytes", proof_bytes);
}
