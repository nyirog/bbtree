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

    pub fn get_ref(&self, key: i32) -> Option<&i32> {
        match key.cmp(&self.key) {
            Equal => {
                Some(&self.value)
            },
            Less => {
                self.left.as_ref().map_or(None, |left|left.get_ref(key))
            },
            Greater => {
                self.right.as_ref().map_or(None, |right|right.get_ref(key))
            }
        }
    }

//    pub fn get_mut(&self, key: i32) -> Option<&mut i32> {
//        match key.cmp(&self.key) {
//            Equal => {
//                Some(&mut *self.value)
//            },
//            Less => {
//                self.left.as_ref().map_or(None, |left|left.get_mut(key))
//            },
//            Greater => {
//                self.right.as_ref().map_or(None, |right|right.get_mut(key))
//            }
//        }
//    }

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

    pub fn get_ref(&self, key: i32) -> Option<&i32> {
        match self.root {
            None => None,
            Some(ref node) => node.get_ref(key),
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
fn test_tree_get_ref_returns_none_for_unknown_key() {
    let tree = BinTree::new();
    assert_eq!(tree.get_ref(42), None);
}

#[test]
fn test_tree_get_ref_returns_a_reference() {
    let value: i32 = 56;
    let mut tree = BinTree::new();

    tree.insert(42, value);

    assert_eq!(tree.get_ref(42), Some(&value));
}

#[test]
fn test_tree_get_ref_for_the_bigger_sub_tree() {
    let value: i32 = 56;
    let mut tree = BinTree::new();

    tree.insert(42, 21);
    tree.insert(66, value);

    assert_eq!(tree.get_ref(66), Some(&value));
}

#[test]
fn test_tree_get_ref_for_the_smaller_sub_tree() {
    let value: i32 = 56;
    let mut tree = BinTree::new();

    tree.insert(42, 21);
    tree.insert(33, value);

    assert_eq!(tree.get_ref(33), Some(&value));
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

#[test]
fn test_node_get_ref_returns_node_for_unknoen_key() {
    let value: i32 = 56;
    let node = Node::new(42, value);

    assert_eq!(node.get_ref(36), None);
}

#[test]
fn test_node_get_ref_returns_a_reference() {
    let value: i32 = 56;
    let node = Node::new(42, value);

    assert_eq!(node.get_ref(42), Some(&value));
}

#[test]
fn test_node_get_ref_for_the_bigger_sub_node() {
    let value: i32 = 56;
    let mut node = Node::new(42, 20);

    node.insert(66, value);

    assert_eq!(node.get_ref(66), Some(&value));
}

#[test]
fn test_node_get_ref_for_the_smaller_sub_node() {
    let value: i32 = 56;
    let mut node = Node::new(42, 20);

    node.insert(33, value);

    assert_eq!(node.get_ref(33), Some(&value));
}

// #[test]
// fn test_get_mut_returns_onne_for_unknown_key() {
//     let node = Node::new(42, 21);
// 
//     assert_eq!(node.get_mut(21), None);
// }

// #[test]
// fn test_node_get_mut() {
//     let mut value: i32 = 56;
//     let node = Node::new(42, 20);
// 
//     assert_eq!(node.get_mut(33), Some(&mut value));
// }


fn main() {
    let mut tree = BinTree::new();
    tree.insert(42, 56);

    println!("Under {} lives {}", 42, tree.get(42).unwrap());
}
