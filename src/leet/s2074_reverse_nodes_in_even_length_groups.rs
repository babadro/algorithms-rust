use crate::helpers::linked_list::ListNode;

struct Solution;

// todo: implement simpler solution
// https://leetcode.com/problems/reverse-nodes-in-even-length-groups/submissions/927627175/
impl Solution {
    pub fn reverse_even_length_groups(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut result_tail = &mut result;

        let (mut reverted_group_head, mut g_count) = (None, 0);

        let mut expected_len = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            if g_count == 0 {
                if let Some(group) = reverted_group_head.replace(node) {
                    _ = result_tail.insert(group);
                    while let Some(node) = result_tail {
                        result_tail = &mut node.next;
                    }
                }
            } else {
                // revert
                reverted_group_head.insert(node).next = reverted_group_head.take();
            }

            g_count += 1;

            // revert again
            if (g_count == expected_len || !head.is_some()) && g_count % 2 == 1 {
                let mut tail_head = reverted_group_head.take();
                while let Some(mut node) = tail_head {
                    tail_head = node.next.take();
                    reverted_group_head.insert(node).next = reverted_group_head.take();
                }
            }

            if g_count == expected_len {
                (g_count, expected_len) = (0, expected_len + 1)
            }
        }

        if let Some(tail) = reverted_group_head {
            _ = result_tail.insert(tail);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::linked_list::{linked_list_to_vector, vector_to_linked_list};

    #[test]
    fn test_reverse_even_length_groups() {
        let test_cases = [
            (vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
            (vec![1, 2, 3], vec![1, 3, 2]),
            (vec![1, 2, 3, 4], vec![1, 3, 2, 4]),
            (vec![1, 2, 3, 4, 5], vec![1, 3, 2, 5, 4]),
            (vec![1, 2, 3, 4, 5, 6], vec![1, 3, 2, 4, 5, 6]),
            (vec![1, 2, 3, 4, 5, 6, 7], vec![1, 3, 2, 4, 5, 6, 7]),
            (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![1, 3, 2, 4, 5, 6, 8, 7]),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 3, 2, 4, 5, 6, 7, 8, 9],
            ),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![1, 3, 2, 4, 5, 6, 10, 9, 8, 7],
            ),
        ];

        for (input, want) in test_cases {
            let head = vector_to_linked_list(&input);
            let got_list = Solution::reverse_even_length_groups(head);

            let got = linked_list_to_vector(&got_list);

            assert_eq!(got, want, "{:?}", input);
        }
    }
}

/*
let test_cases =
 */
