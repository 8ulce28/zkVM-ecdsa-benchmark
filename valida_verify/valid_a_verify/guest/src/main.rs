#![no_main]
valida_rs::entrypoint!(main);

use k256::{
    ecdsa::{Signature, VerifyingKey, signature::Verifier},
    PublicKey,
};


pub fn main() {

    // Hardcoded values for the sake of speed

    let msg_bytes: [u8; 32] = [
    127, 208, 178, 212, 164, 94, 120, 175, 39, 225, 46, 45, 227, 107, 13, 145, 174, 219, 233, 37, 221, 212, 183, 86, 115, 138, 76, 85, 204, 251, 36, 114
    ];

    let sig_bytes: [u8; 64] = [212, 166, 211, 4, 172, 98, 92, 151, 208, 29, 167, 35, 79, 189, 243, 239, 68, 111, 51, 157, 110, 79, 165, 26, 34, 18, 245,
    102, 133, 31, 57, 245, 3, 47, 222, 123, 51, 210, 97, 168, 127, 109, 204, 82, 102, 81, 216, 33, 152, 11, 233, 175, 194, 209, 94, 79, 198, 25, 126, 19,
    0, 167, 230, 194];

    let pk_bytes: [u8; 65] = [4, 27, 101, 113, 74, 52, 27, 87, 250, 155, 204, 248, 200, 197, 247, 98, 146, 214, 118, 138, 156, 247, 222, 62, 150, 189,
    182, 136, 27, 15, 40, 235, 171, 128, 240, 173, 245, 198, 203, 32, 95, 136, 104, 228, 76, 217, 29, 215, 26, 79, 123, 89, 124, 137, 44, 56, 76, 184,
    112, 123, 170, 226, 70, 75, 169];

    let signature = Signature::from_bytes((&sig_bytes).into()).expect("Invalid signature format");
    let public_key = PublicKey::from_sec1_bytes(&pk_bytes).expect("Invalid public key");
    let verify_key = VerifyingKey::from(public_key);

    verify_key
        .verify(&msg_bytes, &signature)
        .expect("Signature failed to verify");
}
