use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};

fn main() {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext: Vec<u8> = cipher
        .encrypt(&nonce, b"plaintext message".as_ref())
        .unwrap();
    let plaintext: Vec<u8> = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    println!("{}", String::from_utf8(plaintext).unwrap());

    let r = OsRng.next_u64();
    let r1 = OsRng.next_u64();
    println!("{}", r < r1);
}
