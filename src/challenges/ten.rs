extern crate openssl;

use challenges::nine;
use challenges::two;

use self::openssl::symm::{Cipher, Crypter, Mode};

const BYTES_PER_BLOCK: usize = 16;

pub fn decode_cbc_128(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut padded_message = vec![0; ciphertext.len()];

    let mut previous_ciphertext_block = iv;

    for block_number in 0..(ciphertext.len() / BYTES_PER_BLOCK) {
        let offset = block_number * BYTES_PER_BLOCK;
        let block = &ciphertext[offset..(offset + BYTES_PER_BLOCK)];
        let decoded = decode_aes_128_block(block, key);
        let original = two::xor_vectors(&decoded, &previous_ciphertext_block);
        padded_message[offset..(offset + BYTES_PER_BLOCK)].clone_from_slice(&original);
        previous_ciphertext_block = block;
    }

    nine::pkcs_unpad(&padded_message).to_vec()
}

pub fn encode_cbc_128(message: &[u8], key: &[u8], iv: Vec<u8>) -> Vec<u8> {
    let padded = nine::pkcs_pad(message, BYTES_PER_BLOCK);
    let mut ciphertext = vec![0; padded.len()];

    let mut previous_cipherblock = iv;

    for block_number in 0..(padded.len() / BYTES_PER_BLOCK) {
        let offset = block_number * BYTES_PER_BLOCK;
        let block = &padded[offset..(offset + BYTES_PER_BLOCK)];
        let cbc_block = two::xor_vectors(block, &previous_cipherblock);
        let encoded_block = encode_aes_128_block(&cbc_block, key);
        ciphertext[offset..(offset + BYTES_PER_BLOCK)].clone_from_slice(&encoded_block);
        previous_cipherblock = encoded_block;
    }

    ciphertext
}

fn decode_aes_128_block(cipherblock: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    decrypter.pad(false);

    // This needs to have space for an extra block or else `update` panics.
    let mut message = vec![0; 2 * BYTES_PER_BLOCK];

    decrypter
        .update(cipherblock, &mut message)
        .expect("Decrypt error.");

    decrypter.finalize(&mut message).expect("Decrypt error");

    message.truncate(BYTES_PER_BLOCK);
    message
}

fn encode_aes_128_block(message_block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None).unwrap();
    encrypter.pad(false);

    // This needs to have space for an extra block or else `update` panics.
    let mut cipherblock = vec![0; 2 * BYTES_PER_BLOCK];

    encrypter
        .update(message_block, &mut cipherblock)
        .expect("Encrypt error");

    encrypter.finalize(&mut cipherblock).expect("Encrypt error");

    cipherblock.truncate(BYTES_PER_BLOCK);
    cipherblock
}

#[cfg(test)]
mod tests {
    extern crate base64;

    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn should_decode_the_example() {
        let ciphertext = ciphertext_from_file();
        let key = b"YELLOW SUBMARINE";
        let iv = vec![0; BYTES_PER_BLOCK];

        let decoded = decode_cbc_128(&ciphertext, key, &iv);

        assert_eq!(message_from_file(), decoded);
    }

    #[test]
    fn should_encode_the_example() {
        let message = message_from_file();
        let key = b"YELLOW SUBMARINE";
        let iv = vec![0; BYTES_PER_BLOCK];

        let ciphertext = ciphertext_from_file();

        assert_eq!(ciphertext, encode_cbc_128(&message, key, iv));
    }

    fn ciphertext_from_file() -> Vec<u8> {
        let mut base64_encoded = String::new();
        let mut file = File::open("data/10.txt").expect("File not found.");
        file.read_to_string(&mut base64_encoded)
            .expect("Read error");

        base64_encoded = str::replace(&base64_encoded, "\n", "");
        base64::decode(&base64_encoded).expect("base64 decode error")
    }

    fn message_from_file() -> Vec<u8> {
        let mut message = Vec::new();
        let mut file = File::open("data/funky.txt").expect("File not found.");
        file.read_to_end(&mut message).expect("Read error");
        message
    }
}
