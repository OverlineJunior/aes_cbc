mod crypto;

use crate::crypto::{decrypt, encrypt};

fn main() {
    let text = "Hello, world!";
    let (cipher, key) = encrypt(&text.as_bytes().to_vec());
    let decrypted_bytes = decrypt(&cipher, &key);
    let decrypted_text = String::from_utf8(decrypted_bytes).unwrap();
    println!("Original: {text}");
    println!("Encrypted: {cipher:?}");
    println!("Decrypted: {decrypted_text:?}");
}
