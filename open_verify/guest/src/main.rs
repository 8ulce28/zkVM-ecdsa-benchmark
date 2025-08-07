#![no_std]
#![no_main]

// use openvm::io::reveal_u32;
use ecdsa_core::{signature::{hazmat::PrehashVerifier}, RecoveryId, Signature};
use openvm_ecc_guest::ecdsa::VerifyingKey;
use openvm_k256::{Secp256k1, Secp256k1Point};

openvm::init!();
openvm::entry!(main);

fn main() {

    // Hardcoded bytes for the sake of speed

    let msg_hash: [u8; 32] = [127, 208, 178, 212, 164, 94, 120, 175, 39, 225, 46, 45, 227, 107, 13, 145, 174,
    219, 233, 37, 221, 212, 183, 86, 115, 138, 76, 85, 204, 251, 36, 114];

    let sig: [u8; 64] = [56, 88, 24, 56, 21, 204, 69, 193, 12, 201, 192, 190, 42, 154, 246, 131, 71, 178,
    160, 96, 122, 247, 10, 91, 19, 16, 168, 66, 142, 117, 65, 54, 50, 62, 147, 140, 55, 85, 59, 106, 229, 60, 163,
    88, 126, 250, 167, 92, 223, 122, 98, 145, 137, 29, 235, 204, 189, 68, 223, 173, 101, 150, 68, 118];

    let real_sig = Signature::from_bytes((&sig).into()).expect("invalid signature");

    let recovery_id = RecoveryId::from_byte(1).unwrap();

    let recovered_vk = VerifyingKey::<Secp256k1>::recover_from_prehash_noverify(&msg_hash, &sig, recovery_id).unwrap();

    let verified = recovered_vk.verify_prehash(&msg_hash, &real_sig).is_ok();

    // let recovered_bytes = recovered_vk.to_sec1_bytes(false);  // Introducing additional check
    // assert_eq!(recovered_bytes, expected_bytes);  // where expected_bytes are the bytes of the known public key

    // reveal_u32(if verified { 1 } else { 0 }, 0);  // For feedback
}
