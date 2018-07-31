use std::cmp::Ordering;
use std::collections::BinaryHeap;

use challenges::three;
use english_score::english_score;
use hamming;

const BEST_KEYSIZES_TO_COLLECT: usize = 4;
const MAX_KEYSIZE: usize = 40;
const BLOCKS_TO_CHECK: usize = 6;

pub fn decode_repeating_key_xor(ciphertext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let keysizes = find_best_keysizes(ciphertext);

    let (_, _, message_and_key) = keysizes
        .iter()
        .map(|keysize| {
            let (message, key) = decode_with_keysize(ciphertext, *keysize);
            let score = english_score(&message);
            let negative_keysize = -(*keysize as i64); // So that the lowest keysize is the max.
            (score, negative_keysize, (message, key))
        })
        .max()
        .unwrap();

    message_and_key
}

fn decode_with_keysize(ciphertext: &[u8], keysize: usize) -> (Vec<u8>, Vec<u8>) {
    let blocks = construct_blocks(ciphertext, keysize);

    let mut message: Vec<u8> = vec![0; ciphertext.len()];
    let mut key: Vec<u8> = vec![0; keysize];

    for (block_number, block) in blocks.iter().enumerate() {
        let (decoded_block, block_key, _score) = three::decode_xor(&block);

        key[block_number] = block_key;

        for (byte_in_block, byte) in decoded_block.iter().enumerate() {
            let message_byte_number = (byte_in_block * blocks.len()) + block_number;
            message[message_byte_number] = *byte;
        }
    }

    (message, key)
}

fn construct_blocks(ciphertext: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let block_len = ciphertext.len() / keysize;
    let mut blocks: Vec<Vec<u8>> = vec![Vec::with_capacity(block_len); keysize];

    for (i, e) in ciphertext.iter().enumerate() {
        blocks[i % keysize].push(*e);
    }

    blocks
}

fn find_best_keysizes(ciphertext: &[u8]) -> Vec<usize> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct KeysizeDistance {
        keysize: usize,
        distance: usize,
    }

    impl Ord for KeysizeDistance {
        fn cmp(&self, other: &KeysizeDistance) -> Ordering {
            other
                .distance
                .cmp(&self.distance)
                .then_with(|| self.keysize.cmp(&other.keysize))
        }
    }

    impl PartialOrd for KeysizeDistance {
        fn partial_cmp(&self, other: &KeysizeDistance) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let max_keysize = if MAX_KEYSIZE > (ciphertext.len() / BLOCKS_TO_CHECK) {
        ciphertext.len() / BLOCKS_TO_CHECK
    } else {
        MAX_KEYSIZE
    };

    let mut keysize_distances: BinaryHeap<KeysizeDistance> = (2..max_keysize)
        .map(|keysize| KeysizeDistance {
            keysize: keysize,
            distance: distance_for_keysize(ciphertext, keysize),
        })
        .collect();

    let mut results = Vec::with_capacity(BEST_KEYSIZES_TO_COLLECT);

    for _ in 0..BEST_KEYSIZES_TO_COLLECT {
        let keysize = keysize_distances.pop().unwrap().keysize;
        results.push(keysize);
    }

    results
}

fn distance_for_keysize(ciphertext: &[u8], keysize: usize) -> usize {
    let mut total_edit_distance = 0;

    for block_number in 0..(BLOCKS_TO_CHECK - 1) {
        let first_offset = block_number * keysize;
        let second_offset = first_offset + keysize;
        let first_block = &ciphertext[first_offset..second_offset];
        let second_block = &ciphertext[second_offset..(second_offset + keysize)];

        total_edit_distance += hamming::distance(first_block, second_block);
    }

    let precision = 100;
    let normalized_distance = (total_edit_distance * precision) / keysize;

    normalized_distance
}

#[cfg(test)]
mod tests {
    extern crate base64;
    extern crate hex;

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::str::from_utf8;

    #[test]
    fn should_decode_the_example() {
        let ciphertext = ciphertext_from_file();

        let (decoded_bytes, key_bytes) = decode_repeating_key_xor(&ciphertext);
        let guessed_key = from_utf8(&key_bytes).expect("UTF-8 error");
        let decoded_message = from_utf8(&decoded_bytes).expect("UTF-8 error");

        let expected_key = "Terminator X: Bring the noise";
        let expected_message = solution_from_file();

        assert_eq!(expected_key, guessed_key);
        assert_eq!(expected_message, decoded_message)
    }

    fn ciphertext_from_file() -> Vec<u8> {
        let mut base64_encoded = String::new();
        let mut file = File::open("data/6.txt").expect("File not found.");
        file.read_to_string(&mut base64_encoded)
            .expect("Read error");

        base64_encoded = str::replace(&base64_encoded, "\n", "");

        base64::decode(&base64_encoded).expect("base64 decode error")
    }

    fn solution_from_file() -> String {
        let mut message = String::new();
        let mut file = File::open("data/6_solution.txt").expect("File not found.");
        file.read_to_string(&mut message).expect("Read error");
        message
    }

    #[test]
    fn should_decode_the_message_from_five() {
        let encoded_hex =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
             a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let ciphertext = hex::decode(&encoded_hex).expect("hex error");

        let (decoded_bytes, key_bytes) = decode_repeating_key_xor(&ciphertext);
        let guessed_key = from_utf8(&key_bytes).expect("UTF-8 error");
        let decoded_message = from_utf8(&decoded_bytes).expect("UTF-8 error");

        let expected_key = "ICE";
        let expected_message = "Burning 'em, if you ain't quick and nimble\n\
                                I go crazy when I hear a cymbal";

        assert_eq!(expected_key, guessed_key);
        assert_eq!(expected_message, decoded_message)
    }
}
