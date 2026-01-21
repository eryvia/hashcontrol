use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

struct Values {
    pepper: String,
    max_len: usize,
    hashes: HashSet<String>,
}

// Emitting function
fn emit(line: &str) {
    let mut e = io::stderr().lock();
    let _ = writeln!(e, "{line}");
    let _ = e.flush();
}

fn get_values() -> Values {
    let pepper = "cajovna-2025-".to_string();
    let max_len = 8; //maximum length of passwords to brute force

    let hashes: HashSet<_> = vec![
        "81620f5ccec6b6ab1364cd17f91a74b2100944487c12b9eb48b1e2307154199a".to_string(),
        "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af".to_string(),
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".to_string(),
        "a46c44824d7700c2e7d2c86e17025be0f28482410f5e770d661b32e30164d503".to_string(),
        "1d2719873cff37e405223beefc3a4e13c25e871ea45328b1eee38a4f54f16fd1".to_string(),
        "69afe5ab59ff69d74d844e63aaa00126784236d6761ebc6ca3b0c8e9ead9d1a2".to_string(),
        "de90a8cdb9f93dfe464eaff087e9fc499538cbc75446cc614b032c73602758a9".to_string(),
        "c7b74803d65c28312b589c068b508c6b034b92625ef11e3362944824aabafba7".to_string(),
        "6e293357be91f025093b00e9e2d7992b32bb7027b38d4f8e8cd07b2572a784de".to_string(),
        "c77e65d03954a519375563dc0265aff5cbf21339a18fec76960a1396cd4c2a6c".to_string(),
        "22afa49c2c3b54cfaf76d41c0f2f03e0a158b2a5f167cd7e5beaccae7eeabc53".to_string(),
        "086f62d6b7473b0520bd7dfe05ebd9aaa85206405671c7a70fa9fccfdb77949d".to_string(),
        "d5ffe58ce2524439faa3a3b08baca953d1219d95096f0553fbf9bcac7d0453ff".to_string(),
        "83ac3adcfe9d740ee7daff1eca787cab6b32661523072828d2fb9bc488ae050e".to_string(),
        "b5e0d9288aed1f56be8907eb20abc31d959e4de25fd72abee355e24846dc374f".to_string(),
        "8ea952ef7897d22e2e60c1cb37f0cfbf58e428bebe27134f22ffa2d69590e565".to_string(),
        "9700312d8b4b197598831807a00d7fee1398ae8e7ea5beabef1adf93fdb144e7".to_string(),
        "35f0149cde02f694ca28cd91cd21d899b49d60978999d1afdf63d283c9938b41".to_string(),
        "9005c2ec3bb91632e54bae83f8d0bce503ca9d876c7e6a37969c2dd5300f29e8".to_string(),
        "c57b6290de6758e1835875628b813f5f1b7b57d2f5099c12382ea68aee6d68c6".to_string(),
        "7195cd37287686fea1d2f377fdeee5f0d3654283bf664a38cd7ba50e5cf3a219".to_string(),
        "c600335efb7524a1e8be77e17d777c0681389309c64c41c9465dbe4f33b0183b".to_string(),
        "ab73a8da571083a9e3e7863a549df4e41b386356b33fc4ff271d9bfb4ae22ba3".to_string(),
        "0eed5e923693634707c8d70c8123924ec5cad7cff464ab182fa3c6a69cfee020".to_string(),
        "8f7a59bd491a985ba65b5b30d5c7ab82e73cd59c6ed37ff75c650f53e85837e8".to_string(),
        "f02ea9a508a026131ef0ac4880803bd43c77933651f7fb0925266606107aeb70".to_string(),
        "78162aefdfd64862631de675e892e4e49abb3890b210a44b6b6489f3192e77aa".to_string(),
        "c91f6f792fc2457aea0d4d936fa26be8b369d04e8391dcd06a13358237613334".to_string(),
        "e9b09e00ea25611357ec91aec6c63d67d611b80d347681f0399a799e27b49a0d".to_string(),
        "2a982fbc50b6980e358e6a28e2324cb1832258f59462fb07e8353921f69d2d5a".to_string(),
        "1d55af3fb1f922151b059f82359e4f60b0a6846dfb4b50bba7b0b0a21963c7d8".to_string(),
        "3c7b7ff05bd9fdc02cc47a469782d3ffbb2d5d32dc767a92efdd88e7360aa437".to_string(),
        "f0480203de58aec6e2fb0827128fdebb88f01f7123d114889c27f5b70237f425".to_string(),
        "0f7c7c064e424f57843c25656249acf70006210bcbee2120db030960279714ae".to_string(),
        "7c3ee6210d24c76b5c807d577abec65116305bb2c425d96cdc6df13a9d4b4338".to_string(),
        "3ee6cf0a0a7e041cfdc4101085c780011d351245bf451372d87d6f6ae04d463b".to_string(),
        "412cd0f542949bbf596416ee127781ed2d58597641283bfcdcd9ccbdbb360e88".to_string(),
        "767886b769a8c528c98ad871d01c6022919ab3cfc0a5e844731eb33d0fa47ed1".to_string(),
        "ae272789fe245723ba8fdc1d5e6ce33b9323a642ddc0c591476e7d5a69c199cc".to_string(),
        "28ad10535a92b6786bdd83e61b26465d9f5623a4319b9c24827449539788e9d7".to_string(),
        "39e1ae3cc5abc4c4483a9b530f27ff852a82dd5b718bd49651548cc529354d0c".to_string(),
        "03a60022e3476ad9ff98a744db7699e5553c099867bb39c2a4593d5fdbeb434f".to_string(),
        "32c1d4d523aa63aa03c3d99f6af03feb0245feeac79def287f0be5a02a3530c6".to_string(),
        "60e509af126e8873ccd4f2a8b3813f09dd100a69a256ea4f998363ab277933af".to_string(),
        "73be671643cac51caa9d9b57df0821b6a7a67aab914fbebe3b264e500182a75f".to_string(),
        "76213e7dd86559b2833fb5efc55c0bb02d960f56120cc1a78e294bb6c18f58b0".to_string(),
        "53c47ee8975346668140e02efaaff40fbcfcbc853b30d8447e464b52ec76f9dc".to_string(),
        "83cb8120d8bf5ece78a3c57a17284415d20389f179bc8b2948d843b981dd57aa".to_string(),
        "e355e0290b13e2b7f041c0260dfb523e6676784a7249eaf7520b39d08f1aa2be".to_string(),
        "6196e3e0c665bf85a791ba6d4ed0ad453b9659ff63a4ab0deed3bec7e60c863a".to_string(),
        "728da61619ba48b3003a6a3743edf4d4da9be4497f557c83c4763fdabb7b8172".to_string(),
        "148194bd577d3e51843b3c7ea516115a0b4e1fd9caeb7b9fd186003c857c6965".to_string(),
        "b1a21d96357614c89b0739639b47c6d68049ea0e30cd7991aa3f3efac0503ace".to_string(),
        "ef698a8672d3c4dc199b0defa7a703e0ec9dadefe8950d90798a92d0592fcefc".to_string(),
        "dd7db55a8ef28974e9b4170751218efba1aab948eaf2aa179e41393bce2a0795".to_string(),
        "1b449b3dcab2a3801719ec42fc4af4cd2016ef44c7eee16c717e5da26c97a91f".to_string(),
        "c4ddc6ae09253b0e0e7943d2c9222848363a4a0e9e8b1b88120a0c4bce5a4cd6".to_string(),
        "47e457671c51efe38d3c6e0b5734dd44c1ffc7db830fbc959405f09e0bdc7027".to_string(),
        "636e900b6d134f1f3a59d5c284103eb549a266f6faeba34a69ccffebdea06ba8".to_string(),
        "f4093342c8ac54f040383762688ff9fb18e54cbbaea688934aaee3f82aa2016b".to_string(),
        "8a38c9a8ef1fccc6b8da80aa34e01a3eff5b8ef1cca0a43884d302360ac38edf".to_string(),
        "649dea42b86fe2230d98b1c82e4e4bfcd7992d3f267cea6f39125865fb42151b".to_string(),
        "673385f8f607692ce4e7789f3bea203d00170fb63edbff51fc956360fcfcca67".to_string(),
        "eb957f4fa95f19cd4c9fdfd6be4c6e1f4537eec76da125460e79bfdeedd57fe8".to_string(),
        "bd920d37bb92f6b09dce32f7b25070f09e24341ae6e2a6df75ca0faf0ccd0676".to_string(),
        "dc5fa6a1132671e4978ad27bd532eaa91e0501ee954088db23723ebc4ddf1f17".to_string(),
        "f52bc5da3143aaf7fa94406b5f3fdac69f0f9221541f7595aab857f94cf80dea".to_string(),
        "4e2955c07cd733d69ed158e7a64e122715d7e83d155e75ede2cc26244fff1ef6".to_string(),
        "b093e82194abe3379c00f132d1bb5490405aad9b708aab0de94901fc7390ce7d".to_string(),
        "b00fbfefebd3af3abf506292c5ed614e3296f44b3c7a4abc4ad5301d936c3bc3".to_string(),
        "ea628f0d991a0d9625392dcc5f3607b0eb991abdc5ce57c22c1d976067c74bc6".to_string(),
        "fce262f465a40e62e8d00bd8c6856bed26f4ca4f7010c834bdc3ee0dedcc77c4".to_string(),
        "74bb3e598dee2f71439c42b5bceef3324894e15527dcf242d89c4a166cd71f3f".to_string(),
        "ce53431ec3ef3a06afe23591420baa3116c3dd988c4f8042882c7275f1ce16a7".to_string(),
        "f853c6c7f610eca081c0e77d4a4971996896ea1e81fb68d077900f5cc5328d41".to_string(),
        "5c692075bcec6d82e9de06ad5de16e6848c372625e69d8c620827b395e2c22f7".to_string(),
        "57cf2011d28d607d9ee82ee57b1b7250556ee6147d92f1084728c662046a8d9e".to_string(),
        "c32f1a595400b517780ed9962cdedd17df48214ab661ee7f95d5ed8c7d121a9c".to_string(),
        "f44322d898b25be323d0e6ab46ecdbdcbe98856078ebb71412f7b03612722493".to_string(),
    ]
    .into_iter()
    .collect();

    Values {
        pepper,
        max_len,
        hashes,
    }
}

//Hashing function
fn hash_bytes(word: &[u8], pepper: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pepper.as_bytes());
    hasher.update(word);
    hex::encode(hasher.finalize())
}

fn brute_force_length(len: usize, info: &Values, checked: &AtomicU64, start: Instant) {
    emit(&format!(r#"{{"type":"length_start","len":{len}}}"#));
    //this is what i was commenting at the end of the file
    ALPHABET.par_iter().for_each(|&first| {
        let mut buf = vec![0u8; len];
        buf[0] = first;
        generate_recursive(&mut buf, 1, len, info, checked, start);
    });

    emit(&format!(r#"{{"type":"length_done","len":{len}}}"#));
}

//Posiution fn mainly
fn generate_recursive(
    buf: &mut [u8],
    pos: usize,
    target_len: usize,
    info: &Values,
    checked: &AtomicU64,
    start: Instant,
) {
    if pos == target_len {
        //let n = checked.fetch_add(1, Ordering::Relaxed) + 1;
        let hash = hash_bytes(buf, &info.pepper);

        /* emting progress removed  ... for now aldlam,dwnmka
        if n % PRINT_EVERY == 0 {
            let secs = start.elapsed().as_secs_f64().max(0.000001);
            let speed = (n as f64 / secs).round() as u64;
            emit(&format!(
                r#"{{"type":"progress","checked":{n},"speed":{speed}}}"#
            ));
        }*/

        if info.hashes.contains(&hash) {
            let word = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
            emit(&format!(
                r#"{{"type":"found","word":"{word}","hash":"{hash}"}}"#
            ));
        }

        return;
    }

    for &ch in ALPHABET {
        buf[pos] = ch;
        generate_recursive(buf, pos + 1, target_len, info, checked, start);
    }
}

fn main() {
    let info = get_values();
    let checked = AtomicU64::new(0);
    let start = Instant::now();

    emit(&format!(
        r#"{{"type":"start","pepper":"{}","max_len":{}}}"#,
        info.pepper, info.max_len
    ));

    for len in 1..=info.max_len {
        brute_force_length(len, &info, &checked, start);
    }

    emit(&format!(
        r#"{{"type":"done","checked":{},"ms":{}}}"#,
        checked.load(Ordering::Relaxed),
        start.elapsed().as_millis()
    ));
}

//This code is mainly mine, i had help only for those workers (dedicating threads from cpu, i was running tests on Ryzem 7 2700, so noot much -> 8 cores 16 threads)
