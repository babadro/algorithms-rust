use crate::helpers::linked_list::ListNode;
use std::ops::Deref;

struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        slow.cloned()
    }
}

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
