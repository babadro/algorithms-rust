use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut matched = 0;
        let mut freq : HashMap<u8, i32> = HashMap::new();
        for char in s1.as_bytes() {
            let count = freq.entry(*char).or_insert(0);
            *count += 1;
        }

        for endChar in s2.as_bytes() {
            if freq.contains_key(endChar) {
                let count = freq.entry(*endChar).or_default();
                *count -= 1;

                if *count == 0 {
                    matched += 1;
                }
            }

            if matched == freq.len() {
                return true
            }

            let start = end - s1.len() + 1;
            if start >= 0 {

            }
        }

        true
    }
}