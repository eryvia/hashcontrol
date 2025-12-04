use sha2::{Digest, Sha256};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

struct Values {
    pepper: String,
    index: i32,
    starter_hash: Vec<char>,
    hash: String,
}

fn get_values() -> Values {
    let pepper: String = "cajovna-2025-".to_string();
    let index: i32 = 5;
    let starter_hash: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let hash: String =
        "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af".to_string();

    Values {
        pepper,
        index,
        starter_hash,
        hash,
    }
}

fn main() {
    let info: Values = get_values();

    for x in info.starter_hash {
        println!("{}", x);
    }
}
