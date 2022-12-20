struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut triplets: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            Solution::find_triplet(nums[i], &nums[i + 1..], &mut triplets)
        }

        triplets
    }

    fn find_triplet(num: i32, nums: &[i32], triplets: &mut Vec<Vec<i32>>) {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = num + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                triplets.push(vec![num, nums[left], nums[right]]);

                left += 1;
                right -= 1;

                while left > 0 && left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }

                while right < nums.len() - 1 && right > left && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        // todo
        let res = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    }
}
