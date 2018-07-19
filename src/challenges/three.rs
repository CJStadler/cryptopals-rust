extern crate hex;

pub fn decode_xor(encoded: &[u8]) -> String {
    let mut best = String::new();
    let mut best_score = 0.0;

    for key in 0..127 {
        let decoded_vec = decode_with_key(encoded, key);
        match String::from_utf8(decoded_vec) {
            Ok(decoded) => {
                let score = english_score(&decoded);

                if score > best_score {
                    best = decoded;
                    best_score = score;
                }
            },
            Err(_) => ()
        }
    }

    best
}

fn decode_with_key(encoded: &[u8], key: u8) -> Vec<u8> {
    encoded.iter().map(|e| e ^ key).collect()
}

fn english_score(text: &str) -> f64 {
    let english_char = ' ';
    let char_count = text.chars().
        fold(0, |sum, c| {
            if c == english_char {
                sum + 1
            } else {
                sum
            }
        });

    char_count as f64 / text.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_score_english() {
        assert!(english_score("This is some english") > english_score("ad97yqhadfe7q2h"));
    }

    #[test]
    fn should_decode_the_example() {
        let encoded = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
        let expected = "Cooking MC\'s like a pound of bacon";
        assert_eq!(expected, decode_xor(&encoded));
    }
}
