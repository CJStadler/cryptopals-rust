pub fn pkcs_pad(message: &[u8], block_length: usize) -> Vec<u8> {
    let len = message.len();
    let mut padded = message.to_vec();

    let last_block_length = len % block_length;
    let padding_size = (block_length - last_block_length) as u8;

    for _ in 0..padding_size {
        padded.push(padding_size);
    }

    padded
}

pub fn pkcs_unpad(padded_message: &[u8]) -> &[u8] {
    match padded_message.last() {
        Some(padding_size) => {
            let new_length = padded_message.len() - (*padding_size as usize);
            &padded_message[0..new_length]
        }
        None => padded_message,
    }
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;

    #[test]
    fn should_pad_the_message_with_the_number_of_padding_bytes() {
        let message = vec![1, 2, 3];

        assert_eq!(vec![1, 2, 3, 3, 3, 3], pkcs_pad(&message, 3));
        assert_eq!(vec![1, 2, 3, 1], pkcs_pad(&message, 4));
        assert_eq!(vec![1, 2, 3, 1], pkcs_pad(&message, 2));
        assert_eq!(vec![1, 2, 3, 5, 5, 5, 5, 5], pkcs_pad(&message, 8));

        assert_eq!(
            b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec(),
            pkcs_pad(b"YELLOW SUBMARINE", 20)
        );
    }

    #[test]
    fn should_unpad_the_message_based_on_the_last_byte() {
        let message = vec![1, 2, 3];

        assert_eq!(message, pkcs_unpad(&vec![1, 2, 3, 3, 3, 3]));
        assert_eq!(message, pkcs_unpad(&vec![1, 2, 3, 1]));
        assert_eq!(message, pkcs_unpad(&vec![1, 2, 3, 5, 5, 5, 5, 5]));
        assert_eq!(
            b"YELLOW SUBMARINE",
            pkcs_unpad(b"YELLOW SUBMARINE\x04\x04\x04\x04")
        );
    }
}
