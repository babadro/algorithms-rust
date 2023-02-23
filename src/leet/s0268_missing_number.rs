pub struct Solution;

impl Solution {
    // cycling sort
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            while nums[i] < nums.len() as i32 && nums[i] != i as i32 {
                let j = nums[i] as usize;
                nums.swap(i, j);
            }
        }

        for i in 0..nums.len() {
            if nums[i] != i as i32 {
                return i as i32;
            }
        }

        return nums.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        let test_cases = vec![
            (vec![], 0),
            (vec![1], 0),
            (vec![0], 1),
            (vec![0, 1], 2),
            (vec![0, 2], 1),
            (vec![10, 9, 8, 3, 2, 4, 5, 6, 1, 0], 7),
        ];

        for (nums, want) in test_cases {
            let got = Solution::missing_number(nums.clone());
            assert_eq!(got, want, "{:?}", nums)
        }
    }
}
