use std::cmp;
use std::collections::{vec_deque, VecDeque};

pub struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        let mut intervals = VecDeque::from(intervals);

        while intervals.len() > 0 && intervals[0][1] < new_interval[0] {
            res.push(intervals.pop_front().unwrap())
        }

        while intervals.len() > 0 && new_interval[1] >= intervals[0][0] {
            new_interval[0] = cmp::min(new_interval[0], intervals[0][0]);
            new_interval[1] = cmp::max(new_interval[1], intervals[0][1]);

            intervals.pop_front();
        }

        res.push(new_interval);

        while intervals.len() > 0 {
            res.push(intervals.pop_front().unwrap())
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        struct TestCase {
            intervals: Vec<Vec<i32>>,
            new_interval: Vec<i32>,
            want: Vec<Vec<i32>>,
        }

        let test_cases = vec![
            TestCase {
                intervals: vec![vec![1, 3], vec![6, 9]],
                new_interval: vec![2, 5],
                want: vec![vec![1, 5], vec![6, 9]],
            },
            TestCase {
                intervals: vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16],
                ],
                new_interval: vec![4, 8],
                want: vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            },
            TestCase {
                intervals: vec![vec![1, 5]],
                new_interval: vec![2, 3],
                want: vec![vec![1, 5]],
            },
            TestCase {
                intervals: vec![vec![1, 5]],
                new_interval: vec![2, 7],
                want: vec![vec![1, 7]],
            },
        ];

        for tc in test_cases {
            let got = Solution::insert(tc.intervals.clone(), tc.new_interval.clone());
            assert_eq!(got, tc.want, "{:?}", tc.intervals);
        }
    }
}
