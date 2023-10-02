//merge two sorted list
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1 == None && list2 == None {
            return None;
        }
        if list2 == None {
            return list1;
        }
        let mut list1 = list1;
        let mut list2 = list2;
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        while list1.is_some() && list2.is_some() { 
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                *cur = list1.take();
                list1 = cur.as_mut().unwrap().next.take();
            } else {
                *cur = list2.take();
                list2 = cur.as_mut().unwrap().next.take();
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        if list1.is_some() {
            *cur = list1;
        } else {
            *cur = list2;
        }
        head
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: None })) }));
    let list2 = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: None })) }));
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(merged.clone().unwrap().val, 1);
    assert_eq!(merged.clone().unwrap().next.unwrap().val, 1);
    assert_eq!(merged.clone().unwrap().next.unwrap().next.unwrap().val, 2);
    assert_eq!(merged.unwrap().next.unwrap().next.unwrap().next.unwrap().val, 3);
}