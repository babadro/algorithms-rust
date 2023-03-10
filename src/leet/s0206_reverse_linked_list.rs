use crate::helpers::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(mut node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut boxed_node) = node {
            node = boxed_node.next;
            boxed_node.next = prev;
            prev = Some(boxed_node);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::linked_list::{linked_list_to_vector, vector_to_linked_list};

    #[test]
    fn test_reverse_list() {
        let test_cases = [
            (vec![2, 4, 6, 8, 10], vec![10, 8, 6, 4, 2]),
            (vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
            (vec![1], vec![1]),
        ];

        for (input, want) in test_cases {
            let head = vector_to_linked_list(&input);

            let got_list = Solution::reverse_list(head);

            let got = linked_list_to_vector(&got_list);

            assert_eq!(got, want, "{:?}", input)
        }
    }
}
