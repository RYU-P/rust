fn main() {
    let mut list = linked_list::new();
    list.insert(1);
    list.insert(2);
}

pub struct linked_list {
    head: Option<Box<node>>,
    tail: Option<Box<node>>,
} 
pub struct node {
    data: i32,
    next: Option<Box<node>>,
}

impl linked_list {
    pub fn new() -> Self {
        linked_list {head: None, tail: None}
    }

    fn insert(&mut self, data: i32) {      
        if let Some(mut head_node) = self.head.as_mut() {
            while(head_node.next.is_some()) {
                head_node = head_node.next.as_mut().unwrap(); //really understand why its this way
            }
            head_node.next = Some(Box::new(node::new(data)));
        } else {
            self.head = Some(Box::new(node::new(data))); 
        }
    }
}

impl node {
    pub fn new(data:i32) -> node {
        node { data: data, next: None }
    }
}













