use powdr::Session;
use std::{fs, path::PathBuf};

fn main() {
    env_logger::init();

    let size: u32 = 18;

    // Initiating the Powdr session
    let mut session = Session::builder()
        .guest_path("./guest")
        .out_path("powdr-target")
        .chunk_size_log2(size.clone())  // The default value is 20 but can be changed to 18, 19
        .build();

    session.run();

    session.prove();

    // Powdr creates chunks in powdr-target to generate proof
    let chunk_num = 5 * (1 << (20 - size));
    let mut total_size = 0;

    // Extracting proof size
    for i in 0..chunk_num {
        let mut proof_path = PathBuf::from("./powdr-target");
        proof_path.push(format!("chunk_{}", i));
        proof_path.push("guest_proof.bin");

        let bytes = fs::read(&proof_path).unwrap();
        total_size += bytes.len();
    }

    println!("Total proof size: {} bytes", total_size);
}
