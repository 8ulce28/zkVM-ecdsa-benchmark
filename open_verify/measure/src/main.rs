use std::{fs, path::PathBuf};

fn main() {
    let mut proof_path: PathBuf = PathBuf::from("../guest");
    proof_path.push("guest.app.proof");
    let bytes: Vec<u8> = fs::read(&proof_path).unwrap();
    println!("Proof size: {}", bytes.len());
}
