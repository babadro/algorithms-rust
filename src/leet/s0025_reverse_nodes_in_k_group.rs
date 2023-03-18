use crate::helpers::linked_list::ListNode;
use std::ptr;

struct Solution;

// todo: https://leetcode.com/problems/reverse-nodes-in-k-group/solutions/2969229/rust-o-n-o-1-solution-0ms-beats-100-no-dummy-heads-no-recursion/
// or https://leetcode.com/problems/reverse-nodes-in-k-group/solutions/3098897/rust-o-1-mem-safe-moving-nodes-0ms/
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut new_head = None;
        let mut prev = &mut new_head;

        while cur.is_some() {
            let mut len = 0;
            while let Some(mut n) = cur.take() {
                cur = n.next.take();
                n.next = prev.take();
                *prev = Some(n);
                len += 1;
                if len == k {
                    break;
                }
            }

            if len == k {
                while prev.is_some() {
                    prev = &mut prev.as_mut().unwrap().next;
                }
            } else {
                cur = prev.take();
                while let Some(mut n) = cur.take() {
                    cur = n.next.take();
                    n.next = prev.take();
                    *prev = Some(n);
                }
            }
        }

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::linked_list::{linked_list_to_vector, vector_to_linked_list};

    #[test]
    fn test_reverse_k_group() {
        let test_cases = [
            (vec![1, 2, 3], 3, vec![3, 2, 1]),
            (vec![1, 2, 3, 4, 5], 2, vec![2, 1, 4, 3, 5]),
            (vec![1, 2, 3, 4, 5], 3, vec![3, 2, 1, 4, 5]),
            (vec![1, 2, 3], 1, vec![1, 2, 3]),
            (vec![1, 2, 3], 4, vec![1, 2, 3]),
            (vec![1, 2, 3, 4], 2, vec![2, 1, 4, 3]),
        ];

        for (input, k, want) in test_cases {
            let head = vector_to_linked_list(&input);
            let got_list = Solution::reverse_k_group(head, k);

            let got = linked_list_to_vector(&got_list);

            assert_eq!(got, want, "{:?} {:?}", input, k);
        }
    }
}
