pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut freq: [i32; 26] = [0; 26];

        let (mut left, mut most_freq, mut res) = (0, 0, 0);

        let s = s.as_bytes();

        for right in 0..s.len() {
            let right_char = s[right] as usize - 'A' as usize;
            freq[right_char] += 1;
            most_freq = max(most_freq, freq[right_char]);

            if (right - left) as i32 + 1 - most_freq > k {
                let left_char = s[left] as usize - 'A' as usize;
                freq[left_char] -= 1;
                left += 1;
            }

            res = max(res, (right - left + 1) as i32)
        }

        res
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        struct TestCase {
            s: String,
            k: i32,
            want: i32,
        }

        let test_cases = vec![
            TestCase {
                s: "ABAB".to_string(),
                k: 2,
                want: 4,
            },
            TestCase {
                s: "AABABBA".to_string(),
                k: 1,
                want: 4,
            },
            TestCase {
                s: "AAABBBB".to_string(),
                k: 0,
                want: 4,
            },
            TestCase {
                s: "ABBCCCDDDD".to_string(),
                k: 0,
                want: 4,
            },
            TestCase {
                s: "AABCDEAA".to_string(),
                k: 0,
                want: 2,
            },
        ];

        for tc in test_cases {
            let got = Solution::character_replacement(tc.s, tc.k);
            assert_eq!(got, tc.want);
        }
    }
}
