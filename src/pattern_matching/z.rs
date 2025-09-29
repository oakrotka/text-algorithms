/// Returns a Z array, where `Z[i]` is the length of longest common prefix of `text` and
/// `text[i..]`
pub(crate) fn compute_z_array(text: &str) -> Vec<usize> {
    // computes the length of the longest common prefix of `text` substrings
    // starting in `i` and `j`, where `i <= j`
    let common_pref_len = |i: usize, j: usize| {
        debug_assert!(i <= j);
        text.as_bytes()[i..]
            .iter()
            .zip(text.as_bytes()[j..].iter())
            .position(|(c1, c2)| c1 != c2)
            .unwrap_or(text.len() - j)
    };

    let mut z = Vec::with_capacity(text.len());
    z.push(text.len());

    let (mut l, mut r) = (0, 0); // prefix box (substring corresponding to a prefix of `text`)
    for i in 1..text.len() {
        if i >= r {
            // we are outside the prefix box, so we must compute it from scratch
            let pref_len = common_pref_len(0, i);
            z.push(pref_len);
            (l, r) = (i, i + pref_len);
        } else if z[i - l] >= r - i {
            // we are inside the box, but the current position is also a prefix of the string whose
            // length may potentially extend past the box
            let pref_len = r - i + common_pref_len(r - i, r);
            z.push(pref_len);
            (l, r) = (i, i + pref_len);
        } else {
            // we are inside the box, so we use the previously computed corresponding value
            z.push(z[i - l]);
        }
    }

    z
}

// Pattern matching with a simple optimization using the Z array
pub fn z_pattern_match(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() || text.is_empty() {
        return Vec::new();
    }

    let z = compute_z_array(&format!("{pattern}\0{text}"));

    z[pattern.len() + 1..]
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == pattern.len())
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod z_array_tests {
    use crate::pattern_matching::z::compute_z_array;

    #[test]
    fn no_match() {
        let text = "the five boxing wizards jump quickly";
        let expected = vec![0; text.len() - 1];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }

    #[test]
    fn simple_match() {
        let text = "regret";
        let expected = vec![0, 0, 2, 0, 0];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }

    #[test]
    fn simple_matches() {
        let text = "proappropriation";
        let expected = vec![0, 0, 0, 1, 3, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }

    #[test]
    fn out_box_match() {
        let text = "ABACABABA";
        let expected = vec![0, 1, 0, 3, 0, 3, 0, 1];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }

    #[test]
    fn in_box_match() {
        let text = "riroriro";
        let expected = vec![0, 1, 0, 4, 0, 1, 0];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }

    #[test]
    fn scream() {
        let text = "AAAAAAA";
        let expected = vec![6, 5, 4, 3, 2, 1];

        let z = compute_z_array(text);
        assert_eq!(z[1..], expected);
    }
}

test_pattern_matcher!(z_pattern_match);
