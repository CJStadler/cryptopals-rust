extern crate base64;
extern crate hex;

pub fn hex_to_base64(hex: &str) -> String {
    let decoded = hex::decode(hex).unwrap();
    base64::encode(&decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(output, hex_to_base64(input));
    }
}
