#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vector_to_linked_list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = None;

    for num in v.iter().rev() {
        let mut new_node = Box::new(ListNode::new(*num));
        new_node.next = node;
        node = Some(new_node);
    }

    node
}

pub fn linked_list_to_vector(node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    let mut curr_node = node;

    while curr_node.is_some() {
        res.push(curr_node.as_ref().unwrap().val);

        curr_node = &curr_node.as_ref().unwrap().next;
    }

    res
}

mod tests {
    use super::*;

    #[test]
    fn test_vector_to_linked_list() {
        let test_cases: Vec<Vec<i32>> = vec![vec![], vec![1], vec![1, 2]];

        for tc in test_cases.iter() {
            let mut node = vector_to_linked_list(tc);

            for num in tc.iter() {
                assert!(node.is_some());
                assert_eq!(node.as_ref().unwrap().val, *num, "num: {}", num);

                node = node.unwrap().next;
            }
        }
    }

    #[test]
    fn test_linked_list_to_vector() {
        let test_cases: Vec<Vec<i32>> = vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3]];

        for tc in test_cases.iter() {
            let head = vector_to_linked_list(tc);

            let res = linked_list_to_vector(&head);

            assert_eq!(&res, tc);

            println!("{:?}", res)
        }
    }
}
