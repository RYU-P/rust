//question reorder the linked list
//input given is a sorted list L0 -> L1 -> ... -> Ln-1 -> Ln
    //1 -> 2 -> 3 -> 4 -> 5
//output should be L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...
    //1 -> 5 -> 2 -> 4 -> 3

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution:

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {

    }
}

fn main() {
    println!("Hello, world!");
}
