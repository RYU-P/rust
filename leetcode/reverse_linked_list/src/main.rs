#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
   pub val: i32,
   pub next: Option<Box<Node>>
 }
 
impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node {
            next: None,
            val,
        }
    }
 }

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
        let mut temp;
        let mut prev: Option<Box<Node>> = None;    
        while head.is_some() {
            temp = head.clone().unwrap().next;
            head.unwrap().next = prev;
            prev = head;
            temp = head.clone().unwrap().next.take();
            let mut head_mut = head.as_mut().unwrap();
            head_mut.next = prev.take();
            prev = head.take();
            head = temp;
        }
        head
    }
}

fn main() {
    println!("Hello, world!");
}
