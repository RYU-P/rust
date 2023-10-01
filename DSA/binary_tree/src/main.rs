use std::collections::BTreeSet;

// struct for a binary tree node
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
// binary tree struct
struct BinaryTree {
    root: Option<Box<Node>>,
}
fn main() {
    println!("Hello, world!");
}
