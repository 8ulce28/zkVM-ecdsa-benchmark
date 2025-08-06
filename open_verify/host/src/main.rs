use std::sync::Arc;
use std::path::PathBuf;
use std::time::Instant;
use eyre::Result;
use openvm_build::GuestOptions;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::AppProver,
    Sdk, StdIn,
};
use openvm_stark_sdk::config::{baby_bear_poseidon2::BabyBearPoseidon2Engine, fri_params::FriParameters};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setup - Initiating the VM, sdk, and vm executable
    let vm_config = SdkVmConfig::builder()
        .system(Default::default())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .build();

    let sdk = Sdk::new();

    let mut guest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    guest_path.push("../guest");
    let target_path = guest_path.to_str().unwrap();

    let guest_opts = GuestOptions::default();
    let elf = sdk.build(
        guest_opts,
        target_path,
        &Default::default(),
    )?;

    let exe = sdk.transpile(elf, vm_config.transpiler());
    let exe = match exe {
    Ok(exe) => {
        println!("Transpilation completed!");
        exe
    }
    Err(e) => {
        eprintln!("Transpilation failed: {:?}", e);
        return Err(e.into());
    }
    };

    let stdin = StdIn::default();

    println!("Execution started!");
    let _output = sdk.execute(exe.clone(), vm_config.clone(), stdin.clone())?;

    // Setting parameters
    let app_log_blowup = 2;
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let app_config = AppConfig::new(app_fri_params, vm_config);

    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe)?;

    let app_pk = Arc::new(sdk.app_keygen(app_config)?);

    println!("Proof 1 started!");
    let start = Instant::now();
    let _proof = sdk.generate_app_proof(app_pk.clone(), app_committed_exe.clone(), stdin.clone())?;
    let app_prover = AppProver::<_, BabyBearPoseidon2Engine>::new(
        app_pk.app_vm_pk.clone(),
        app_committed_exe.clone(),
    )
    .with_program_name("test_program");
    let finish = start.elapsed();
    println!("Proof generation 1 took: {:#?}", finish);

    println!("Proof 2 started!");
    let start = Instant::now();
    let proof = app_prover.generate_app_proof(stdin.clone());
    let finish = start.elapsed();
    println!("Proof generation 2 took: {:#?}", finish);

    let app_vk = app_pk.get_app_vk();

    println!("Verification started!");
    let start = Instant::now();
    sdk.verify_app_proof(&app_vk, &proof)?;
    let finish = start.elapsed();
    println!("Verification took: {:#?}", finish);

    let proof_bytes = bincode::serialized_size(&proof).unwrap() as usize;
    println!("Proof size: {}", proof_bytes);

    Ok(())
}
