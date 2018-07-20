extern crate hex;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use challenges::three;

pub fn decode_line_in_file(filename: &str) -> Vec<u8> {
    let file = File::open(filename).expect("File not found.");
    let reader = io::BufReader::new(file);

    let mut best = Vec::new();
    let mut best_score = 0;

    for line in reader.lines() {
        let encoded_bytes = hex::decode(line.unwrap()).expect("Line not valid hex.");
        let (decoded, _key, score) = three::decode_xor(&encoded_bytes);

        if score > best_score {
            best = decoded;
            best_score = score;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn should_find_the_encoded_string() {
        let filename = "data/4.txt";
        let expected = "Now that the party is jumping\n";
        assert_eq!(expected, str::from_utf8(&decode_line_in_file(&filename)).expect("UTF-8 error"));
    }
}
