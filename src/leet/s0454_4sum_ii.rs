use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut sums_count = HashMap::new();

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let count = sums_count.entry(nums1[i] + nums2[j]).or_insert(0);

                *count += 1;
            }
        }

        let mut res = 0;
        for i in 0..nums3.len() {
            for j in 0..nums4.len() {
                let count = sums_count.entry(-(nums3[i] + nums4[j])).or_default();

                res += *count;
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_count() {
        struct TestCase {
            nums1: Vec<i32>,
            nums2: Vec<i32>,
            nums3: Vec<i32>,
            nums4: Vec<i32>,
            want: i32,
        }

        let test_cases = vec![
            TestCase {
                nums1: vec![1, 2],
                nums2: vec![-2, -1],
                nums3: vec![-1, 2],
                nums4: vec![0, 2],
                want: 2,
            },
            TestCase {
                nums1: vec![0],
                nums2: vec![0],
                nums3: vec![0],
                nums4: vec![0],
                want: 1,
            },
        ];

        for case in test_cases {
            assert_eq!(
                case.want,
                Solution::four_sum_count(
                    case.nums1.clone(),
                    case.nums2.clone(),
                    case.nums3.clone(),
                    case.nums4.clone()
                ),
                "nums1: {:?}, nums2: {:?}, nums3: {:?}, nums4: {:?}",
                case.nums1,
                case.nums2,
                case.nums3,
                case.nums4
            );
        }
    }
}
