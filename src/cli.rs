use crate::crypto::Content;
use cliclack::*;
use std::{fs, io, path::Path};

#[derive(Clone, PartialEq, Eq)]
enum ActionKind {
    Encrypt,
    Decrypt,
}

pub enum Action {
    Encrypt { input: Vec<u8> },
    Decrypt { cipher: Vec<u8>, key: Vec<u8> },
}

pub fn run_cli(handler: fn(Action)) -> io::Result<()> {
    clear_screen()?;

    intro("AES Crypto CLI")?;

    let action = read_action()?;

    let (pending_msg, done_msg) = match action {
        Action::Encrypt { .. } => ("Encrypting...", "Encryption complete!"),
        Action::Decrypt { .. } => ("Decrypting...", "Decryption complete!"),
    };

    let spin = spinner();
    spin.start(pending_msg);

    handler(action);

    spin.stop("");

    outro(done_msg)?;

    Ok(())
}

fn read_action() -> io::Result<Action> {
    let action_kind = select("Select an action:")
        .item(ActionKind::Encrypt, "Encrypt", "")
        .item(ActionKind::Decrypt, "Decrypt", "")
        .interact()?;

    let action = match action_kind {
        ActionKind::Encrypt => Action::Encrypt {
            input: read_file("input")?,
        },
        ActionKind::Decrypt => Action::Decrypt {
            cipher: read_file("encrypted")?,
            key: read_file("key")?,
        },
    };

    Ok(action)
}

fn read_file(adjective: &str) -> io::Result<Content> {
    let path: String = input(format!("Enter the path to the {adjective} file:"))
        .placeholder(&format!("Path to {adjective} file"))
        .validate(|input: &String| {
            let path = Path::new(input);
            match (path.exists(), path.is_file()) {
                (false, _) => Err("File does not exist"),
                (_, false) => Err("Path is not a file"),
                (true, true) => Ok(()),
            }
        })
        .interact()?;

    fs::read(path)
}

// fn read_decryption_key() -> io::Result<KeyString> {
// 	let key_str: String = input("Enter the decryption key:")
// 		.placeholder(&format!("{} characters key", key))
// 		.interact()?;

// 	Ok(key_str)
// }
