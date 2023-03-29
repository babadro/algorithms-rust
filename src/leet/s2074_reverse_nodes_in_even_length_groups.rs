use crate::helpers::linked_list::ListNode;

struct Solution;

// todo doesn't work
impl Solution {
    pub fn reverse_even_length_groups(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        head = None;
        let mut prev: &mut Option<Box<ListNode>> = &mut head;

        let mut expected_len = 0;

        while cur.is_some() {
            expected_len += 1;
            let mut actual_len = 0;

            let mut tmp_cur = &cur;
            while tmp_cur.is_some() && actual_len < expected_len {
                tmp_cur = &tmp_cur.as_ref().unwrap().next;

                actual_len += 1
            }

            if actual_len % 2 == 1 {
                // todo
                for _ in 0..actual_len {
                    cur = cur.unwrap().next;
                }

                continue;
            }

            for _ in 0..actual_len {
                let mut n = cur.take().unwrap();
                cur = n.next.take();
                n.next = prev.take();
                *prev = Some(n);
            }

            for _ in 0..actual_len {
                prev = &mut prev.as_mut().unwrap().next;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::linked_list::{linked_list_to_vector, vector_to_linked_list};

    #[test]
    fn test_reverse_even_length_groups() {
        let test_cases = [
            //(vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
            //(vec![1, 2, 3], vec![1, 3, 2]),
            //(vec![1, 2, 3, 4], vec![1, 3, 2, 4]),
            //(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 5, 4]),
            //(vec![1, 2, 3, 4, 5, 6], vec![1, 3, 2, 4, 5, 6]),
            //(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 3, 2, 4, 5, 6, 7]),
            //(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![1, 3, 2, 4, 5, 6, 8, 7]),
            //(
            //    vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            //    vec![1, 3, 2, 4, 5, 6, 7, 8, 9],
            //),
            //(
            //    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            //    vec![1, 3, 2, 4, 5, 6, 10, 9, 8, 7],
            //),
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
