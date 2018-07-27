const CHARS_TO_CHECK: [&'static str; 9] = [" ", "e", "t", "a", "o", "i", "n", "s", "r"];
const CHAR_FREQUENCIES: [usize; 9] = [12, 12, 9, 8, 8, 7, 7, 6, 6];

pub fn english_score(text: &[u8]) -> usize {
    if text.is_ascii() {
        let char_count = text.iter().fold(0, |sum, c| {
            match CHARS_TO_CHECK
                .iter()
                .position(|&x| x == (*c as char).to_lowercase().to_string())
            {
                Some(i) => sum + CHAR_FREQUENCIES[i],
                None => sum,
            }
        });

        char_count / text.len()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_score_english() {
        assert!(english_score(b"This is some english") > english_score(b"ad97yqhadfe7q2h"));
    }
}
