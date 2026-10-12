use rand::rngs::OsRng;
use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;

const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn calculate_checksum(key: &str) -> u16 {
    // CRC-style checksum using polynomial 0x14002.
    // Uses u32 internally to match the Python/C++ style:
    //
    // if v2 & 1:
    //     v2 ^= 0x14002
    // v2 >>= 1
    //
    // Then mask to 16 bits.
    let mut v2: u32 = 0;

    for byte in key.bytes() {
        v2 ^= byte as u32;

        for _ in 0..8 {
            if v2 & 1 != 0 {
                v2 ^= 0x14002;
            }

            v2 >>= 1;
        }

        v2 &= 0xFFFF;
    }

    v2 as u16
}

fn generate_key() -> String {
    let mut rng = OsRng;

    let base: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let checksum = calculate_checksum(&base);

    format!(
        "{}-{}-{}-{}-{:04X}",
        &base[0..4],
        &base[4..8],
        &base[8..12],
        &base[12..16],
        checksum
    )
}

fn output_path(file_name: &str) -> PathBuf {
    // Try to save keys.txt next to the executable.
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join(file_name);
        }
    }

    // Fallback: save in the current working directory.
    PathBuf::from(file_name)
}

fn main() {
    println!("Generating keys...");

    let keys: Vec<String> = (0..10000).map(|_| generate_key()).collect();

    for key in &keys {
        println!("{key}");
    }

    let path = output_path("keys.txt");
    let contents = keys.join("\n") + "\n";

    match fs::write(&path, contents) {
        Ok(()) => {
            println!("\nSaved to:\n{}", path.display());
        }
        Err(e) => {
            eprintln!("\nError writing {}: {}", path.display(), e);
        }
    }

    println!("\nPress Enter to exit...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}