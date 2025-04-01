// Definition for singly-linked list.
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;

    let mut carry = 0;
    let mut p1 = l1;
    let mut p2 = l2;

    // Process both lists while they have nodes or while we have a carry
    while p1.is_some() || p2.is_some() || carry > 0 {
        // Extract values and move to next nodes
        let x = match p1 {
            Some(node) => {
                let val = node.val;
                p1 = node.next;
                val
            }
            None => 0,
        };

        let y = match p2 {
            Some(node) => {
                let val = node.val;
                p2 = node.next;
                val
            }
            None => 0,
        };

        // Calculate sum and carry
        let sum = x + y + carry;
        carry = sum / 10;

        // Create and append new node with the digit value
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
    }

    // Return the result (skip the dummy head)
    dummy_head.next
}

fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &value in vec.iter().rev() {
        let mut node = ListNode::new(value);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        output.push(node.val);
        current = node.next;
    }
    output
}

fn main() {
    let l1 = vec_to_list(vec![2, 4, 3]);
    let l2 = vec_to_list(vec![5, 6, 4]);
    println!("Output: {:?}", list_to_vec(add_two_numbers(l1, l2)));

    let l1 = vec_to_list(vec![0]);
    let l2 = vec_to_list(vec![0]);
    println!("Output: {:?}", list_to_vec(add_two_numbers(l1, l2)));

    let l1 = vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = vec_to_list(vec![9, 9, 9, 9]);
    println!("Output: {:?}", list_to_vec(add_two_numbers(l1, l2)));
}
