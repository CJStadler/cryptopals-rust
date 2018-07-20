extern crate hex;

use english_score::english_score;

pub fn decode_xor(encoded: &[u8]) -> (Vec<u8>, u8, usize) {
    let mut best = Vec::with_capacity(encoded.len());
    let mut best_score = 0;
    let mut best_key = 0;

    for key in 0..127 {
        let decoded = decode_with_key(encoded, key);

        let score = english_score(&decoded);

        if score > best_score {
            best = decoded;
            best_score = score;
            best_key = key;
        }
    }

    (best, best_key, best_score)
}

fn decode_with_key(encoded: &[u8], key: u8) -> Vec<u8> {
    encoded.iter().map(|e| e ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn should_decode_the_example() {
        let encoded = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
        let expected = "Cooking MC\'s like a pound of bacon";
        let (decoded, _, _) = decode_xor(&encoded);
        assert_eq!(expected, str::from_utf8(&decoded).expect("UTF-8 error"));
    }
}
