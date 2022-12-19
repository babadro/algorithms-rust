struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut i = 0;
        let mut j = nums.len() - 1;

        for k in (0..nums.len()).rev() {
            let square_i = nums[i] * nums[i];
            let square_j = nums[j] * nums[j];

            if square_j > square_i {
                res[k] = square_j;
                j -= 1;
            } else {
                res[k] = square_i;
                i += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        let test_cases = [
            (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
            (vec![-7, -3, 2, 3, 11], vec![4, 9, 9, 49, 121]),
        ];

        for (nums, want) in test_cases.iter() {
            assert_eq!(Solution::sorted_squares(nums.clone()), want.clone(),)
        }
    }
}
