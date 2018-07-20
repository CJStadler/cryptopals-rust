extern crate hex;

pub fn repeating_key_xor_encode(message: &[u8], key: &[u8]) -> Vec<u8> {
    let key_len = key.len();
    message.iter().enumerate().map(|(i, e)| {
        let key_index = i % key_len;
        e ^ key[key_index]
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_the_example() {
        let message: Vec<u8> = "Burning 'em, if you ain't quick and nimble\n\
            I go crazy when I hear a cymbal".bytes().collect();
        let key: Vec<u8> = "ICE".bytes().collect();
        let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
            a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let result = repeating_key_xor_encode(&message, &key);
        assert_eq!(expected_hex, hex::encode(result));
    }
}
