#![no_main]
pico_sdk::entrypoint!(main);

use k256::{
    ecdsa::{Signature, VerifyingKey, signature::Verifier},
    PublicKey,
};

fn main() {

    // Feeding hardcoded values for the sake of speed

    let msg_bytes: [u8; 32] = [127, 208, 178, 212, 164, 94, 120, 175, 39, 225, 46, 45, 227, 107, 13, 145, 174,
    219, 233, 37, 221, 212, 183, 86, 115, 138, 76, 85, 204, 251, 36, 114];

    let sig_bytes: [u8; 64] = [56, 88, 24, 56, 21, 204, 69, 193, 12, 201, 192, 190, 42, 154, 246, 131, 71, 178,
    160, 96, 122, 247, 10, 91, 19, 16, 168, 66, 142, 117, 65, 54, 50, 62, 147, 140, 55, 85, 59, 106, 229, 60, 163,
    88, 126, 250, 167, 92, 223, 122, 98, 145, 137, 29, 235, 204, 189, 68, 223, 173, 101, 150, 68, 118];

    let pk_bytes: [u8; 65] = [4, 63, 52, 166, 212, 246, 164, 155, 160, 73, 190, 250, 87, 198, 253, 180, 36, 60, 27,
    126, 62, 172, 250, 42, 150, 8, 76, 84, 193, 181, 77, 180, 211, 65, 200, 153, 158, 220, 21, 41, 198, 56, 122, 9,
    238, 13, 107, 71, 90, 156, 219, 185, 44, 135, 251, 61, 100, 163, 1, 234, 181, 49, 78, 75, 185];

    let signature = Signature::from_bytes((&sig_bytes).into()).expect("Invalid signature format");
    let public_key = PublicKey::from_sec1_bytes(&pk_bytes).expect("Invalid public key");
    let verify_key = VerifyingKey::from(public_key);

    verify_key
        .verify(&msg_bytes, &signature)
        .expect("Signature failed to verify");
}
