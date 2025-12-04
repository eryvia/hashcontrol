use sha2::{Digest, Sha256};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

struct Values {
    pepper: String,
    index: i32,
    starter_hash: Vec<char>,
    hashes: Vec<String>,
}

fn get_values() -> Values {
    let pepper: String = "cajovna-2025-".to_string();
    let index: i32 = 5;
    let starter_hash: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let hashes: Vec<String> = vec![
        "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af".to_string(),
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".to_string(),
    ];

    Values {
        pepper,
        index,
        starter_hash,
        hashes,
    }
}

fn get_hash(value: &str, pepper: &str) -> String {
    let source: String = pepper.to_string() + value;

    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let info: Values = get_values();
    let mut curr_index: i32 = 0;
    let mut curr_value: String = "".to_string();
    let mut control: bool = true;

    while control == true {
        for index in 0..info.index {
            for x in &info.starter_hash {
                curr_value.push(*x);
                control = false;
            }
        }
    }
}
