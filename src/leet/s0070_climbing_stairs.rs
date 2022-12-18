struct Solution {}

impl Solution {
    pub fn climb_stairs_brute_force(n: i32) -> i32 {
        if n == 2 {
            return 2;
        }

        if n == 1 {
            return 1;
        }

        return Solution::climb_stairs_brute_force(n - 1)
            + Solution::climb_stairs_brute_force(n - 2);
    }

    pub fn climb_stairs_top_down(n: i32) -> i32 {
        let mut dp = vec![0].repeat(n as usize + 1);

        return Solution::rec_top_down(&mut dp, n as usize);
    }

    fn rec_top_down(dp: &mut Vec<i32>, n: usize) -> i32 {
        if n == 2 {
            return 2;
        }

        if n == 1 {
            return 1;
        }

        if dp[n] == 0 {
            dp[n] = Solution::rec_top_down(dp, n - 1) + Solution::rec_top_down(dp, n - 2);
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        let test_cases = [(1, 1), (2, 2), (3, 3), (4, 5)];

        for (n, expected) in test_cases {
            assert_eq!(Solution::climb_stairs_top_down(n), expected, "{:?}", n,)
        }
    }
}
