use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::time::Instant;

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
//const ALPHABET: &[u8] = b"abc012";

struct Values {
    pepper: String,
    max_len: usize,
    hashes: HashSet<String>,
}

fn get_values() -> Values {
    let pepper = "cajovna-2025-".to_string();
    let max_len = 6;

    let hashes: HashSet<_> = vec![
        "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af".to_string(),
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".to_string(),
    ]
    .into_iter()
    .collect();

    Values {
        pepper,
        max_len,
        hashes,
    }
}

fn hash_bytes(buf: &[u8], pepper: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pepper.as_bytes());
    hasher.update(buf);
    hex::encode(hasher.finalize())
}

// ---------------- PARALLEL PREFIX BRUTE FORCE -----------------

fn brute_force_length(len: usize, info: &Values) {
    println!("Bruteforcing length {}...", len);

    ALPHABET.par_iter().for_each(|&first_char| {
        // Each worker gets its own buffer
        let mut buf = vec![first_char; len];

        // Recursively brute force the rest of the characters
        generate_recursive(&mut buf, 1, len, info);
    });
}

fn generate_recursive(buf: &mut Vec<u8>, pos: usize, max_len: usize, info: &Values) {
    if pos == max_len {
        let hash = hash_bytes(buf, &info.pepper);

        if info.hashes.contains(&hash) {
            let word = unsafe { String::from_utf8_unchecked(buf.clone()) };
            println!("FOUND: {} -> {}", word, hash);
        }
        return;
    }

    for &ch in ALPHABET {
        buf[pos] = ch;
        generate_recursive(buf, pos + 1, max_len, info);
    }
}

// ------------------- MAIN ---------------------------------
fn main() {
    let info = get_values();

    println!("Starting bruteforce...");
    let start_time = Instant::now();

    for len in 1..=info.max_len {
        brute_force_length(len, &info);
    }

    let elapsed = start_time.elapsed();
    println!("Finished in {:.2?} seconds", elapsed);
}

/*
fn generate_recursive(buf: &mut Vec<u8>, pos: usize, max_len: usize, info: &Values) {
    if pos == max_len {
        let curr_hash = hash_bytes(buf, &info.pepper);

        if info.hashes.contains(&curr_hash) {
            let word = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
            println!("FOUND: {word} -> {curr_hash}");
        }
        return;
    }

    for &ch in ALPHABET {
        buf[pos] = ch;
        generate_recursive(buf, pos + 1, max_len, &info);
    }
}

*/
