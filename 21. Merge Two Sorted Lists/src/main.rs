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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1_vec = list_to_vec(list1.clone());
    let mut list2_vec = list_to_vec(list2);
    list1_vec.append(&mut list2_vec);
    list1_vec.sort();
    vec_to_list(list1_vec)
}

fn main() {
    println!("Merging two sorted linked lists into one sorted linked list:\n");

    let l1 = vec_to_list(vec![1, 2, 4]);
    let l2 = vec_to_list(vec![1, 3, 4]);
    println!(
        "Linked list 1: {:?}\nLinked list 2: {:?}\nOutput: {:?}\n",
        list_to_vec(l1.clone()),
        list_to_vec(l2.clone()),
        list_to_vec(merge_two_lists(l1, l2))
    );

    let l1 = vec_to_list(vec![]);
    let l2 = vec_to_list(vec![]);
    println!(
        "Linked list 1: {:?}\nLinked list 2: {:?}\nOutput: {:?}\n",
        list_to_vec(l1.clone()),
        list_to_vec(l2.clone()),
        list_to_vec(merge_two_lists(l1, l2))
    );

    let l1 = vec_to_list(vec![0]);
    let l2 = vec_to_list(vec![]);
    println!(
        "Linked list 1: {:?}\nLinked list 2: {:?}\nOutput: {:?}\n",
        list_to_vec(l1.clone()),
        list_to_vec(l2.clone()),
        list_to_vec(merge_two_lists(l1, l2))
    );
}
