pub fn distance(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .enumerate()
        .map(|(i, a_element)| {
            let b_element = b[i];
            byte_distance(a_element, &b_element)
        })
        .sum()
}

fn byte_distance(a: &u8, b: &u8) -> usize {
    let mut bytes_different = 0;
    let powers_of_two: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    for power_of_two in powers_of_two.iter() {
        let ones: u8 = power_of_two & 255;

        if (ones & a) != (ones & b) {
            bytes_different += 1;
        }
    }

    bytes_different
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_distance_between_the_example_strings() {
        let a: Vec<u8> = "this is a test".bytes().collect();
        let b: Vec<u8> = "wokka wokka!!!".bytes().collect();

        assert_eq!(37, distance(&a, &b));
    }

    #[test]
    fn should_return_zero_for_equal_vectors() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];

        assert_eq!(0, distance(&a, &b));
    }

    #[test]
    fn should_compute_distance_between_bytes() {
        assert_eq!(1, byte_distance(&0, &1));
        assert_eq!(8, byte_distance(&0, &255));
    }

    #[test]
    fn should_return_zero_for_equal_bytes() {
        assert_eq!(0, byte_distance(&123, &123));
    }
}
