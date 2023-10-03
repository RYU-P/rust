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

struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
      //split the list in half and go from index 0, n, 1, n-1, until it reaches the middle element. it works because last element will always be the middle(index) element.
      //example: length/2=middle, 
      //4/2=2, 1 -> 2 -> 3 -> 4 and 1 -> 4 -> 2 -> 3.
      //5/2=2  1 -> 2 -> 3 -> 4 -> 5 and 1 -> 5 -> 2 -> 4 -> 3, 
      //slow and fast pointer technique:
      //slow pointer moves one node at a time, fast pointer moves two nodes at a time. when fast pointer reaches the end, slow pointer will be at the middle.    
      let mut slow = &mut head.as_mut();
      let mut fast = &mut head.as_mut();
      while fast.is_some() && fast.is_some() {
        let slow = &mut unwrap().next.as_mut();
        let fast = &mut fast.unwrap().next.as_mut().unwrap().next;
      }

    }
}

fn main() {
    println!("Hello, world!");
}
