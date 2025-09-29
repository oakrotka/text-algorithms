/// Matches `pattern` against `text` using a naive method
pub fn naive_pattern_match(text: &str, pattern: &str) -> Vec<usize> {
    let patlen = pattern.len();
    let txtlen = text.len();

    if patlen == 0 || txtlen < patlen {
        return Vec::new();
    }

    text.bytes()
        .enumerate()
        .take(txtlen - patlen + 1)
        .map(|(i, _)| i)
        .filter(|&i| text.is_char_boundary(i) && text[i..i + patlen] == *pattern)
        .collect()
}

test_pattern_matcher!(naive_pattern_match);
