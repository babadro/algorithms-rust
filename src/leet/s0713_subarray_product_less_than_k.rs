struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut product = 1;

        for (right, num) in nums.iter().enumerate() {
            product *= *num;

            while product >= k && left <= right {
                product = product / nums[left];
                left += 1;
            }

            if left <= right {
                res += right - left + 1
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_subarray_product_less_than_k() {
        let test_cases = [(vec![10, 5, 2, 6], 100, 8), (vec![1, 2, 3], 0, 0)];

        for (nums, k, want) in test_cases.iter() {
            let got = Solution::num_subarray_product_less_than_k(nums.clone(), *k);

            assert_eq!(got, *want, "{:?}", nums,)
        }
    }
}

/*
{[]int{10, 5, 2, 6}, 100, 8},
        {[]int{1, 2, 3}, 0, 0},
 */
