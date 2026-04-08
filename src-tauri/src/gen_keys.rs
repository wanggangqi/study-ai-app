use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::SigningKey;

fn main() {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verify_key = signing_key.verifying_key();

    println!("SIGNING_KEY={}", BASE64.encode(signing_key.as_bytes()));
    println!("VERIFY_KEY={}", BASE64.encode(verify_key.as_bytes()));
}