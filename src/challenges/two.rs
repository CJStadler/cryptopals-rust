extern crate hex;

pub fn xor_vectors(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().enumerate().
        map(|(i, &a_element)| a_element ^ b[i]).
        collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let expected = hex::decode("746865206b696420646f6e277420706c6179").unwrap();
        assert_eq!(expected, xor_vectors(&a, &b));
    }
}
