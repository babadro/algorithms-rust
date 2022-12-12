use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn permute_palindrome(s1: String) -> bool {
        let mut m: HashMap<u8, i32> = HashMap::new();

        let mut odd_count = 0;

        for b in s1.bytes() {
            let entry = m.entry(b).or_insert(0);

            *entry += 1;

            if *entry % 2 == 1 {
                odd_count += 1
            } else {
                odd_count -= 1;
            }
        }

        if s1.len() % 2 == 1 {
            return odd_count == 1;
        }

        odd_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permute_palindrome() {
        let test_cases = [
            ("a", true),
            ("ab", false),
            ("", true),
            ("aaa", true),
            ("aaab", false),
            ("abcba", true),
            ("abcde", false),
            ("aabbccddee", true),
            ("aabbccddeeffgghhiijj", true),
            ("", true),
        ];

        for &(s, expected) in test_cases.iter() {
            assert_eq!(
                Solution::permute_palindrome(s.to_string()),
                expected,
                "{:?}",
                s
            )
        }
    }
}
