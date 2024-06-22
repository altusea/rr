use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::chacha20::ChaCha20;
use crypto::symmetriccipher::{Decryptor, Encryptor};

fn main() {
    let key = [0x00u8; 32]; // 256-bit key
    let nonce = [0x00u8; 8]; // 64-bit nonce

    // 初始化 ChaCha20
    let mut cipher = ChaCha20::new(&key, &nonce);

    // 要加密的数据
    let plain_text = "Hello, Rust! This is a ChaCha20 encryption example.";

    // 加密数据
    let mut encrypted = vec![0u8; plain_text.len()];
    {
        let mut write_buffer = WriteBuffer::new(&mut encrypted);
        cipher.encrypt(&mut ReadBuffer::new(plain_text.as_bytes()), &mut write_buffer, true);
    }

    // 解密数据，使用相同的密钥和 nonce
    let mut decrypted = vec![0u8; encrypted.len()];
    let mut cipher = ChaCha20::new(&key, &nonce, 0); // 重新初始化 cipher
    {
        let mut write_buffer = WriteBuffer::new(&mut decrypted);
        cipher.decrypt(&mut ReadBuffer::new(&encrypted), &mut write_buffer, true);
    }

    assert_eq!(plain_text.as_bytes(), &decrypted[..]);

    println!("Original: {}", plain_text);
    println!("Encrypted: {:?}", encrypted);
    println!("Decrypted: {}", String::from_utf8(decrypted).unwrap());
}