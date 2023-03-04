struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            let j = (nums[i] - 1) as usize;
            if j >= 0 && j < nums.len() && nums[j] != nums[i] {
                nums.swap(i, j)
            } else {
                i = i + 1
            }
        }

        for i in 0..nums.len() {
            if nums[i] - 1 != i as i32 {
                return (i + 1) as i32;
            }
        }

        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        let test_cases = [
            (vec![1, 2, 0], 3),
            (vec![3, 4, -1, 1], 2),
            (vec![7, 8, 9, 11, 12], 1),
            (vec![1], 2),
            (vec![1, 1], 2),
        ];

        for (nums, want) in test_cases {
            let got = Solution::first_missing_positive(nums.clone());

            assert_eq!(got, want, "{:?}", nums);
        }
    }
}
