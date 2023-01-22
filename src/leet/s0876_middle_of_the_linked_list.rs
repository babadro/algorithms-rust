use crate::helpers::linked_list::{ListNode, ListNodeRc};
use std::ops::Deref;
use std::rc::Rc;

struct Solution;

impl Solution {
    // two pointers. best algorithm
    // todo 1: avoid cloning
    // options to avoid cloning:
    // a - using RefCell maybe, b - using unsafe, c - using lifetime?
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        slow.cloned()
    }

    // two passes, but without cloning
    pub fn middle_node2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut counter: i32 = 0;

        let mut curr_node = head.as_ref();

        while curr_node.is_some() {
            counter += 1;
            curr_node = curr_node.unwrap().next.as_ref();
        }

        let mut middle = head;
        for _ in 0..counter / 2 {
            middle = middle.unwrap().next;
        }

        middle
    }
}

// todo 2: implement this signature
// pub fn middle_node(head: &Option<Box<ListNode>>) -> &Option<Box<ListNode>>

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::linked_list::vector_to_linked_list;

    #[test]
    fn test_middle_node() {
        let test_cases = [
            (vec![0], 0),
            (vec![0, 1], 1),
            (vec![0, 1, 2], 1),
            (vec![0, 1, 2, 3], 2),
            (vec![0, 1, 2, 3, 4], 2),
        ];

        for (input, want) in test_cases.iter() {
            let head = vector_to_linked_list(input);

            let got = Solution::middle_node(head);

            assert_eq!(got.unwrap().val, *want);
        }
    }
}
