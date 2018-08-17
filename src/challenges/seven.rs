extern crate openssl;

use self::openssl::symm::{decrypt, Cipher};

pub fn decode_aes_128_ecb(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    decrypt(Cipher::aes_128_ecb(), key, None, ciphertext).expect("AES 128 decoding error.")
}

#[cfg(test)]
mod tests {
    extern crate base64;

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::str::from_utf8;

    #[test]
    fn should_decode_the_example() {
        let ciphertext = ciphertext_from_file();
        let key = b"YELLOW SUBMARINE";
        let decoded_bytes = decode_aes_128_ecb(&ciphertext, key);
        let decoded_message = from_utf8(&decoded_bytes).expect("UTF-8 error");

        let expected_message = solution_from_file();

        assert_eq!(expected_message, decoded_message)
    }

    fn ciphertext_from_file() -> Vec<u8> {
        let mut base64_encoded = String::new();
        let mut file = File::open("data/7.txt").expect("File not found.");
        file.read_to_string(&mut base64_encoded)
            .expect("Read error");

        base64_encoded = str::replace(&base64_encoded, "\n", "");

        base64::decode(&base64_encoded).expect("base64 decode error")
    }

    fn solution_from_file() -> String {
        let mut message = String::new();
        let mut file = File::open("data/funky.txt").expect("File not found.");
        file.read_to_string(&mut message).expect("Read error");
        message
    }
}
