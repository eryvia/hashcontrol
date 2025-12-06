use sha2::{Digest, Sha256};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
//const ALPHABET: &[u8] = b"abc012";

struct Values {
    pepper: String,
    max_len: i32,
    hashes: Vec<String>,
}

fn get_values() -> Values {
    let pepper: String = "cajovna-2025-".to_string();
    let max_len: i32 = 4;
    let hashes: Vec<String> = vec![
        "81620f5ccec6b6ab1364cd17f91a74b2100944487c12b9eb48b1e2307154199a".to_string(),
        "35f0149cde02f694ca28cd91cd21d899b49d60978999d1afdf63d283c9938b41".to_string(),
    ];

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

fn main() {
    let info: Values = get_values();

    println!("Starting bruteforce up to length {}…", info.max_len);

    let index = &info.max_len;
    let mut control: bool = true;

    while control == true {
        for x in 0..*index + 1 {
            if (x as usize) == 0 {
                continue;
            }

            generate_all(x as usize, &info);

            print!("Finished all lengths up to: {} \n", x);
        }
        control = false;
    }
}

fn generate_all(max_len: usize, info: &Values) {
    let mut buf = vec![ALPHABET[0]; max_len];
    return generate_recursive(&mut buf, 0, max_len, &info);
}

fn generate_recursive(buf: &mut Vec<u8>, pos: usize, max_len: usize, info: &Values) {
    if pos == max_len {
        let curr_hash = hash_bytes(buf, &info.pepper);
        if info.hashes.contains(&curr_hash) {
            let word = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
            println!("FOUND: {} -> {}", word, curr_hash);
        }
        return;
    }

    for &ch in ALPHABET {
        buf[pos] = ch;
        generate_recursive(buf, pos + 1, max_len, &info);
    }
}
