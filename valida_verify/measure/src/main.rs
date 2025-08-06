use std::{fs, path::PathBuf};

fn main() {
    let proof_path: PathBuf = PathBuf::from("../valid_a_verify/guest/proof");
    let bytes: Vec<u8> = fs::read(&proof_path).unwrap();
    println!("Proof size: {}", bytes.len());
}
