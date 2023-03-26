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
            while let Some(mut n) = cur.take() {
                cur = n.next.take();
                n.next = prev.take();
                *prev = Some(n);
                actual_len += 1;
                if actual_len == expected_len {
                    break;
                }
            }

            if actual_len % 2 == 0 {
                while prev.is_some() {
                    prev = &mut prev.as_mut().unwrap().next;
                }
            } else {
                let mut new_cur = prev.take();
                while let Some(mut n) = new_cur.take() {
                    new_cur = n.next.take();
                    n.next = prev.take();
                    *prev = Some(n);
                }

                if cur.is_some() {
                    let _ = new_cur.insert(cur.take().unwrap());
                    cur = new_cur;
                }
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
            //(vec![1, 2], vec![1, 2]),
            (vec![1, 2, 3], vec![1, 3, 2]),
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
