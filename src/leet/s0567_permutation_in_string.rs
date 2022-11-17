use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut matched = 0;
        let mut freq: HashMap<u8, i32> = HashMap::new();
        for char in s1.as_bytes() {
            let count = freq.entry(*char).or_insert(0);
            *count += 1;
        }

        let s2 = s2.as_bytes();

        for end in 0..s2.len() {
            let end_char = s2[end];
            if freq.contains_key(&end_char) {
                let count = freq.entry(end_char).or_default();
                *count -= 1;

                if *count == 0 {
                    matched += 1;
                }
            }

            if matched == freq.len() {
                return true;
            }

            let start = end as i32 + 1 - s1.len() as i32;
            if start >= 0 {
                let start_char = s2[start as usize];
                if freq.contains_key(&start_char) {
                    let count = freq.entry(start_char).or_default();
                    if *count == 0 {
                        matched -= 1;
                    }

                    *count += 1;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        struct TestCase {
            s1: String,
            s2: String,
            want: bool,
        }

        let test_cases = vec![
            TestCase {
                s1: "ab".to_string(),
                s2: "eidbaooo".to_string(),
                want: true,
            },
            TestCase {
                s1: "ab".to_string(),
                s2: "eidboaoo".to_string(),
                want: false,
            },
            TestCase {
                s1: "ab".to_string(),
                s2: "bac".to_string(),
                want: true,
            },
            TestCase {
                s1: "a".to_string(),
                s2: "a".to_string(),
                want: true,
            },
        ];

        for tc in test_cases {
            let got = Solution::check_inclusion(tc.s1, tc.s2);
            assert_eq!(got, tc.want);
        }
    }
}
