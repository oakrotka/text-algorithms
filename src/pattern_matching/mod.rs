macro_rules! test_pattern_matcher {
    ($matcher:ident) => {
        #[cfg(test)]
        mod macro_tests {
            use super::$matcher;

            #[test]
            fn basic_match() {
                let text = "homeowner";
                let pattern = "meow";
                assert_eq!($matcher(text, pattern), vec![2]);
            }

            #[test]
            fn no_match_smaller_pattern() {
                let text = "longer text";
                let pattern = "pa";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn no_match_exact_len() {
                let text = "piernik";
                let pattern = "wiatrak";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn no_match_existing_letters() {
                let text = "the quick brown fox jumps over the lazy dog";
                let pattern = "wizard";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn no_match_with_prefix() {
                let text = "the quick brown fox jumps over the lazy dog";
                let pattern = "fops";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn prefix_match() {
                let text = "pattern";
                let pattern = "pat";
                assert_eq!($matcher(text, pattern), vec![0]);
            }

            #[test]
            fn suffix_match() {
                let text = "text";
                let pattern = "ext";
                assert_eq!($matcher(text, pattern), vec![1]);
            }

            #[test]
            fn exact_match() {
                let text = "hello";
                let pattern = "hello";
                assert_eq!($matcher(text, pattern), vec![0]);
            }

            #[test]
            fn multiple_matches() {
                let text = "barbarian";
                let pattern = "ba";
                assert_eq!($matcher(text, pattern), vec![0, 3]);
            }

            #[test]
            fn empty_text() {
                let text = "";
                let pattern = "is anyone there?";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn empty_pattern() {
                let text = "I'm scared";
                let pattern = "";
                assert_eq!($matcher(text, pattern), vec![]);
            }

            #[test]
            fn overlapping() {
                let text = "AAAAAA";
                let pattern = "AA";
                assert_eq!($matcher(text, pattern), vec![0, 1, 2, 3, 4]);
            }

            #[test]
            fn multibyte_unicode() {
                let text = "żółć";
                let pattern = "ół";
                assert!(pattern.chars().all(|c| c.len_utf8() > 1));
                assert_eq!($matcher(text, pattern), vec![2]);
            }
        }
    };
}

pub mod naive;
pub mod z;
