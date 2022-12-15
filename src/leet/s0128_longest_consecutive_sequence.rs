use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut n = 0;

        let mut val_to_idx: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            if val_to_idx.contains_key(&num) {
                continue;
            }

            val_to_idx.insert(num, n);
            n += 1;
        }

        let mut u = Uf::new(n);

        for (val, idx) in val_to_idx.iter() {
            if let Some(up_idx) = val_to_idx.get(&(val + 1)) {
                u.union(*idx as usize, *up_idx as usize)
            } else if let Some(down_idx) = val_to_idx.get(&(val - 1)) {
                u.union(*idx as usize, *down_idx as usize)
            }
        }

        let mut res = 0;
        for size in u.size {
            res = max(res, size);
        }

        res
    }
}

struct Uf {
    ids: Vec<i32>,
    size: Vec<i32>,
}

impl Uf {
    fn new(n: i32) -> Self {
        Uf {
            ids: (0..n).collect(),
            size: vec![1].repeat(n as usize),
        }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.ids[i] as usize {
            self.ids[i] = self.ids[self.ids[i] as usize];
            i = self.ids[i] as usize;
        }

        i
    }

    fn union(&mut self, i: usize, j: usize) {
        let i_root = self.root(i);
        let j_root = self.root(j);

        if i_root == j_root {
            return;
        }

        if self.size[i_root] > self.size[j_root] {
            self.ids[i_root] = j_root as i32;
            self.size[j_root] += self.size[i_root]
        } else {
            self.ids[j_root] = i_root as i32;
            self.size[i_root] += self.size[j_root]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let test_cases = [
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];

        for (v, expected) in test_cases.iter() {
            assert_eq!(
                Solution::longest_consecutive(v.clone()),
                *expected,
                "{:?}",
                v
            )
        }
    }
}
