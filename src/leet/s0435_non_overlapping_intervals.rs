struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|v| v[1]);

        let mut j = 0;
        let mut counter = 1;

        for i in 1..intervals.len() {
            if intervals[j][1] <= intervals[i][0] {
                counter += 1;
                j = i;
            }
        }

        return intervals.len() as i32 - counter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_overlap_intervals() {
        let test_cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
            (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]], 1),
            (vec![vec![1, 2], vec![1, 2], vec![1, 2]], 2),
            (vec![vec![1, 2], vec![2, 3]], 0),
        ];

        for (intervals, want) in test_cases {
            assert_eq!(
                Solution::erase_overlap_intervals(intervals.clone()),
                want,
                "intervals: {:?}",
                intervals
            )
        }
    }
}
