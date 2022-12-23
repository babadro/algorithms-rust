pub struct Solution {}

impl Solution {
    pub fn search_triplets(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let mut res = 0;
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                if nums[i] + nums[left] + nums[right] < target {
                    res += right - left;

                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        res as i32
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /*
    {[]int{-1, 0, 2, 3}, 3, 2},
        {[]int{-1, 4, 2, 1, 3}, 5, 4},
     */
    #[test]
    fn test_search_triplets() {
        let test_cases = [(vec![-1, 0, 2, 3], 3, 2), (vec![-1, 4, 2, 1, 3], 5, 4)];

        for (nums, target, want) in test_cases {
            let got = Solution::search_triplets(nums.clone(), target);

            assert_eq!(want, got, "{:?}", nums)
        }
    }
}
