struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut i = (s.len() - 1) as i32;
        let mut j = (t.len() - 1) as i32;

        let mut i_skip = 0;
        let mut j_skip = 0;

        let s = s.as_bytes();
        let t = t.as_bytes();
        while i >= 0 || j >= 0 {
            while i >= 0 {
                let i_char = s[i as usize];
                if i_char == b'#' {
                    i_skip += 1;
                    i -= 1;
                } else if i_skip > 0 {
                    i -= 1;
                    i_skip -= 1;
                } else {
                    break;
                }
            }

            while j >= 0 {
                let j_char = t[j as usize];
                if j_char == b'#' {
                    j_skip += 1;
                    j -= 1;
                } else if j_skip > 0 {
                    j -= 1;
                    j_skip -= 1;
                } else {
                    break;
                }
            }

            if i >= 0 && j >= 0 && s[i as usize] != t[j as usize] {
                return false;
            }

            if (i >= 0) != (j >= 0) {
                return false;
            }

            i -= 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_backspace_compare() {
        let test_cases = vec![
            ("ab#c", "ad#c", true),
            ("ab##", "c#d#", true),
            ("a##c", "#a#c", true),
            ("a#c", "b", false),
            ("y#fo##f", "y#f#o##f", true),
            ("bxj##tw", "bxj###tw", false),
            ("bbbextm", "bbb#extm", false),
        ];

        for (s, t, want) in test_cases {
            let got = Solution::backspace_compare(s.to_string(), t.to_string());
            assert_eq!(got, want, "{:?} {:?}", s, t);
        }
    }
}
