use std::{fs, path::Path};

use anyhow::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};

use solana_inspect::{GREEN, MAGENTA, RESET};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("[idl_parse]");
        eprintln!("Usage: {} $path", args[0]);
        std::process::exit(1);
    }

    let path_str = resolve_path(&args[1]);

    match process_file(&path_str) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("failed to process file {}", e);
            std::process::exit(1);
        }
    }
}

fn resolve_path(input: &str) -> String {
    let path = Path::new(input);

    if path.is_absolute() {
        if path.exists() {
            return input.to_string();
        } else {
            panic!("path does not exist: {}", input);
        }
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let relative_to_root = Path::new(manifest_dir).join("idl").join(input);

    if relative_to_root.exists() {
        return relative_to_root.to_string_lossy().to_string();
    }

    panic!("relative path transform failed");
}

fn process_file(path_str: &str) -> Result<()> {
    println!("resolved path -> {}", path_str);

    let content = fs::read_to_string(path_str)?;
    let idl: Value = serde_json::from_str(&content)?;

    if let Some(instructions) = idl["instructions"].as_array() {
        for instr in instructions {
            let name = instr["name"].as_str().unwrap_or("unknown");
            let discriminator = calculate_discriminator(name);
            let encoded = bs58::encode(&discriminator).into_string();

            println!(
                "fn {MAGENTA}{}{RESET} => {GREEN}{}{RESET} {:?}",
                name, encoded, discriminator
            );
        }
    }

    Ok(())
}

fn calculate_discriminator(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name); // anchor convention
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    let result = hasher.finalize();

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[0..8]);
    discriminator
}
