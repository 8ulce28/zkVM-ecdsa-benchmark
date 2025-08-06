use std::{path::Path, time::Instant, fs};
use tempfile::NamedTempFile;
use valida_vm_api_linux_arm::tmpfile_helper::*;

// The benchmark was run on a MacBook Pro
#[cfg(target_arch = "aarch64")]
use valida_vm_api_linux_arm::{create_valida, ProveStatus, RunStatus, VerifyStatus};

fn main() {

    // Setup
    let the_guest = Path::new("/app/guest_binary");
    let valida = create_valida().unwrap();
    let stdin = NamedTempFile::new().unwrap();
    let stdout = NamedTempFile::new().unwrap();

    // Executing the guest code
    let start = Instant::now();
    let run_status = valida.run(
        &the_guest,
        stdout.as_ref(),
        stdin.as_ref(),
        Default::default(),
    );
    let end = start.elapsed();
    println!("Execution took: {:#?}", end);

    assert_eq!(run_status, RunStatus::TerminatedWithStop);

    // Proof generation
    let proof = NamedTempFile::new().unwrap();

    let start = Instant::now();
    let prove_status = valida.prove(
        &the_guest,
        proof.as_ref(),
        stdin.as_ref(),
        Default::default(),
        Default::default(),
    );
    let end = start.elapsed();
    println!("Proof generation took: {:#?}", end);

    assert_eq!(prove_status, ProveStatus::Success);

    // Verification
    let start = Instant::now();
    let verify_status_correct_statement = valida.verify(
        &the_guest,
        proof.as_ref(),
        stdout.as_ref(),
        Default::default(),
        Default::default(),
    );
    let end = start.elapsed();
    println!("Verification took: {:#?}", end);

    let bytes = fs::metadata(proof.path()).unwrap().len() as usize;
    println!("Proof size: {}", bytes);

    assert_eq!(verify_status_correct_statement, VerifyStatus::Success);
}
