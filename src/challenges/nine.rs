pub fn pkcs_pad(message: &[u8], block_length: usize) -> Vec<u8> {
    let len = message.len();
    let mut padded = message.to_vec();

    if len != block_length {
        let last_block_length = len % block_length;
        let padding_size = (block_length - last_block_length) as u8;

        for _ in 0..padding_size {
            padded.push(padding_size);
        }
    }

    padded
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;

    #[test]
    fn should_pad_the_message_with_the_number_of_padding_bytes() {
        let message = vec![1, 2, 3];

        assert_eq!(message.clone(), pkcs_pad(&message, message.len()));
        assert_eq!(vec![1, 2, 3, 1], pkcs_pad(&message, 4));
        assert_eq!(vec![1, 2, 3, 1], pkcs_pad(&message, 2));
        assert_eq!(vec![1, 2, 3, 5, 5, 5, 5, 5], pkcs_pad(&message, 8));

        assert_eq!(
            b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec(),
            pkcs_pad(b"YELLOW SUBMARINE", 20)
        );
    }
}
