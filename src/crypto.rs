use soft_aes::aes::{aes_dec_cbc, aes_enc_cbc};

pub const KEY_BYTES: usize = 32;
const IV_BYTES: usize = 16;
// ! Explicar motivação.
const PADDING: Option<&str> = Some("PKCS7");

pub type Content = Vec<u8>;
pub type Cipher = Vec<u8>;
pub type Key = [u8; KEY_BYTES];

pub fn encrypt(text: &Content) -> (Cipher, Key) {
    let mut key = [0u8; KEY_BYTES];
    rand::fill(&mut key);

    // CBC needs an IV (initialization vector).
    let mut iv = [0u8; IV_BYTES];
    rand::fill(&mut iv);

    // In order to match with DES, we use CBC mode here as well.
    let ciphertext = aes_enc_cbc(text, &key, &iv, PADDING).unwrap();

    // Full cipher is iv + ciphertext (order matters).
    let cipher = [iv.as_slice(), &ciphertext].concat();

    (cipher, key)
}

pub fn decrypt(cipher: &Cipher, key: &Key) -> Content {
    let (iv, cipher) = cipher.split_at(IV_BYTES);

    aes_dec_cbc(cipher, key, iv.try_into().unwrap(), PADDING).unwrap()
}

// fn generate_key() -> Key {
//     let mut key_bytes = [0u8; KEY_BYTES];
//     rand::fill(&mut key);


// }
