pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut freq: [i32; 26];

        let (mut left, mut most_freq, mut res) = (0, 0, 0);

        let s = s.as_bytes();

        for right in 0..s.len() {
            let rightChar: i32 = s[right] - 'A';
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::character_replacement("".to_string(), 0));
    }
}