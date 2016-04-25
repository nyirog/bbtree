use std::cmp::Ordering::*;

struct Node {
    key: i32,
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {key: key, value: value, left: None, right: None}
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        match key.cmp(&self.key) {
            Equal => {
                Some(self.value)
            },
            Less => {
                self.left.as_ref().map_or(None, |left|left.get(key))
            },
            Greater => {
                self.right.as_ref().map_or(None, |right|right.get(key))
            }
        }
    }

    pub fn insert(&mut self, key: i32, value: i32) {
        match key.cmp(&self.key) {
            Equal => {
                self.value = value
            },
            Less => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(Node::new(key, value)))
                    },
                    Some(ref mut left) => {
                        left.insert(key, value)
                    },
                }
            },
            Greater => {
                match self.left {
                    None => {
                        self.right = Some(Box::new(Node::new(key, value)))
                    },
                    Some(ref mut right) => {
                        right.insert(key, value)
                    },
                }
            }
        }
    }
}

pub struct BinTree {
    root: Option<Node>,
}

impl BinTree {
    pub fn  new() -> Self {
        BinTree{root: None}
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        match self.root {
            None => None,
            Some(ref node) => node.get(key),
        }
    }

    pub fn insert(&mut self, key: i32, value: i32) {
        match self.root {
            None => {
                self.root = Some(Node::new(key, value))
            },
            Some(ref mut node) => node.insert(key, value),
        }
    }
}

#[test]
fn test_get_none() {
    let tree = BinTree::new();
    assert_eq!(tree.get(42), None);
}

#[test]
fn test_insert() {
    let mut tree = BinTree::new();
    tree.insert(42, 56);
    assert_eq!(tree.get(42), Some(56));
}

#[test]
fn test_insert_smaller() {
    let mut tree = BinTree::new();
    tree.insert(42, 56);
    tree.insert(11, 22);
    assert_eq!(tree.get(42), Some(56));
    assert_eq!(tree.get(11), Some(22));
}

#[test]
fn test_insert_bigger() {
    let mut tree = BinTree::new();
    tree.insert(42, 56);
    tree.insert(111, 222);
    assert_eq!(tree.get(42), Some(56));
    assert_eq!(tree.get(111), Some(222));
}


fn main() {
    let mut tree = BinTree::new();
    tree.insert(42, 56);

    println!("Under {} lives {}", 42, tree.get(42).unwrap());
}
