extern crate hex;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use challenges::three;

pub fn decode_line_in_file(filename: &str) -> String {
    let file = File::open(filename).expect("File not found.");
    let reader = io::BufReader::new(file);

    let mut best = String::new();
    let mut best_score = 0.0;

    for line in reader.lines() {
        let encoded_bytes = hex::decode(line.unwrap()).expect("Line not valid hex.");
        let (decoded, score) = three::decode_xor(&encoded_bytes);

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

    #[test]
    fn should_find_the_encoded_string() {
        let filename = "data/4.txt";
        let expected = "Now that the party is jumping\n";
        assert_eq!(expected, decode_line_in_file(&filename))
    }
}
