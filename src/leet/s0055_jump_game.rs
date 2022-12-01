pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last_pos = nums.len() - 1;

        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= last_pos {
                last_pos = i
            }
        }

        last_pos == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        struct TestCase {
            nums: Vec<i32>,
            want: bool,
        }

        let test_cases = vec![
            TestCase {
                nums: vec![1, 0, 2],
                want: false,
            },
            TestCase {
                nums: vec![1],
                want: true,
            },
            TestCase {
                nums: vec![0],
                want: true,
            },
            TestCase {
                nums: vec![1, 0],
                want: true,
            },
            TestCase {
                nums: vec![3, 0, 0, 0],
                want: true,
            },
            TestCase {
                nums: vec![0, 0],
                want: false,
            },
            TestCase {
                nums: vec![2, 3, 1, 1, 4],
                want: true,
            },
        ];

        for tc in test_cases {
            let got = Solution::can_jump(tc.nums.clone());
            assert_eq!(tc.want, got, "{:?}", tc.nums)
        }
    }
}
