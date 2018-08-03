const BYTES_PER_BLOCK: usize = 16;

pub fn find_ecb_ciphertext(ciphertexts: &[Vec<u8>]) -> usize {
    let (_score, index) = ciphertexts
        .iter()
        .enumerate()
        .map(|(i, ciphertext)| (unique_blocks_count(&ciphertext), i))
        .min()
        .unwrap();
    index
}

fn unique_blocks_count(ciphertext: &[u8]) -> usize {
    let mut blocks = break_into_blocks(ciphertext);
    blocks.sort();
    blocks.dedup();
    blocks.len()
}

fn break_into_blocks(ciphertext: &[u8]) -> Vec<&[u8]> {
    let mut blocks = Vec::new();

    for block_num in 0..(ciphertext.len() / BYTES_PER_BLOCK) {
        let offset = block_num * BYTES_PER_BLOCK;
        blocks.push(&ciphertext[offset..(offset + BYTES_PER_BLOCK)]);
    }

    blocks
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn should_find_the_ciphertext_encoded_with_ecb() {
        let file = File::open("data/8.txt").expect("File not found.");
        let reader = io::BufReader::new(file);

        let ciphertexts: Vec<Vec<u8>> = reader
            .lines()
            .map(|line| hex::decode(line.unwrap()).expect("Line not valid hex."))
            .collect();

        let result_index = find_ecb_ciphertext(&ciphertexts);
        let expected_index = 132;

        assert_eq!(expected_index, result_index);
    }
}
