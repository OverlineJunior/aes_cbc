mod crypto;
mod cli;

use std::{fs, io};
use crate::{cli::{Action, run_cli}, crypto::{decrypt, encrypt}};

fn execute_action(action: Action) {
    match action {
        Action::Encrypt { input } => {
            let (cipher, key) = encrypt(&input);
            fs::write("encrypted.bin", &cipher).expect("Failed to write cipher to file");
            fs::write("key.hex", key).expect("Failed to write key to file");
        }
        Action::Decrypt { cipher, key } => {
            let decrypted = decrypt(&cipher, &key.try_into().expect("Key does not fit"));
            fs::write("decrypted.txt", &decrypted).expect("Failed to write decrypted content to file");
        }
    }
}

fn main() -> io::Result<()> {
    run_cli(execute_action)
}
