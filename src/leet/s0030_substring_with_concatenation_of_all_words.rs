use std::collections::HashMap;

pub struct Solution {}

// todo very slow...
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut freq: HashMap<String, i32> = HashMap::new();
        for word in words.iter() {
            let count = freq.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        let w_len = words[0].len();
        let mut res: Vec<i32> = vec![];
        for i in 0..(s.len() - words.len() * w_len) + 1 {
            let mut visited: HashMap<String, i32> = HashMap::new();
            for j in 0..words.len() {
                let start = i + j * w_len;
                let end = start + w_len;
                let word: &str;
                if end <= s.len() - 1 {
                    word = &s[start..end];
                } else {
                    word = &s[start..]
                }

                let count = freq.entry(word.to_string()).or_default();
                if *count == 0 || *count == *visited.entry(word.to_string()).or_default() {
                    break;
                }

                let visited_count = visited.entry(word.to_string()).or_insert(0);
                *visited_count += 1;

                if j == words.len() - 1 {
                    res.push(i as i32);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        struct TestCase {
            s: String,
            words: Vec<String>,
            want: Vec<i32>,
        }

        let test_cases = vec![
            TestCase {
                s: "barfoothefoobarman".to_string(),
                words: vec!["foo".to_string(), "bar".to_string()],
                want: vec![0, 9],
            },
            TestCase {
                s: "wordgoodgoodgoodbestword".to_string(),
                words: vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string(),
                ],
                want: vec![],
            },
            TestCase {
                s: "barfoofoobarthefoobarman".to_string(),
                words: vec!["bar".to_string(), "foo".to_string(), "the".to_string()],
                want: vec![6, 9, 12],
            },
        ];

        for tc in test_cases {
            let got = Solution::find_substring(tc.s, tc.words);
            assert_eq!(got, tc.want);
        }
    }
}
