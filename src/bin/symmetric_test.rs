use chacha20poly1305::aead::{Aead, Generate, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

fn main() {
    let key = Key::generate();
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = Nonce::generate(); // 96-bits; unique per message
    let ciphertext: Vec<u8> = cipher
        .encrypt(&nonce, b"plaintext message".as_ref())
        .unwrap();
    let plaintext: Vec<u8> = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    println!("{}", String::from_utf8(plaintext).unwrap());

    let r = rand::random::<u64>();
    let r1 = rand::random::<u64>();
    println!("{}", r < r1);
}
