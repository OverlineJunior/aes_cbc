mod crypto;
mod cli;

use std::{fs, io};
use crate::{cli::{Action, run_cli}, crypto::{decrypt, encrypt}};

fn execute_action(action: Action) {
    match action {
        Action::Encrypt { raw_input } => {
            let (cipher, key_hex) = {
                let (c, k) = encrypt(&raw_input);
                (c, hex::encode(k))
            };

            fs::write("encrypted.bin", &cipher).expect("Failed to write cipher to file");
            fs::write("key.hex", &key_hex).expect("Failed to write key to file");
        }
        Action::Decrypt { raw_cipher, raw_key } => {
            let key: crypto::Key = hex::decode(raw_key)
                .expect("Invalid hex in decryption key")
                .as_slice()
                .try_into()
                .expect("Invalid key length");

            let decrypted = decrypt(&raw_cipher, &key);
            fs::write("decrypted.txt", &decrypted).expect("Failed to write decrypted content to file");
        }
    }
}

fn main() -> io::Result<()> {
    run_cli(execute_action)
}
