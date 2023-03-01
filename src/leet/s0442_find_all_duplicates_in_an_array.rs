struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        for i in 0..nums.len() {
            let visited_idx = nums[i].abs() as usize - 1;

            if nums[visited_idx] < 0 {
                res.push(nums[i].abs() as i32);
            }

            nums[visited_idx] = -nums[visited_idx];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        let test_cases = [
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2], vec![1]),
            (vec![1], vec![]),
        ];

        for (nums, want) in test_cases {
            let got = Solution::find_duplicates(nums.clone());

            assert_eq!(got, want, "{:?}", nums)
        }
    }
}
