struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow: usize = 0;
        let mut fast: usize = 0;

        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;

            if slow == fast {
                break;
            }
        }

        slow = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }

        return slow as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        let test_cases = vec![(vec![1, 3, 4, 2, 2], 2), (vec![3, 1, 3, 4, 2], 3)];

        for (nums, want) in test_cases {
            let got = Solution::find_duplicate(nums.clone());

            assert_eq!(got, want, "{:?}", nums);
        }
    }
}
