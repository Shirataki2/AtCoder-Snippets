#![allow(dead_code)]
use cargo_snippet::snippet;

use std::cmp::Ordering;

// Rustのコードの練習用
// 平衡じゃないし削除もないのでいつ使うかわからん

#[snippet("btree")]
struct Node<T>
where
    T: Ord + Copy
{
    value: T,
    left: Box<BTree<T>>,
    right: Box<BTree<T>>,
}

#[snippet("btree")]
impl<T> Node<T>
where
    T: Ord + Copy
{
    fn new(value: T) -> Self {
        Self {
            value: value,
            left: Box::new(BTree::Empty),
            right: Box::new(BTree::Empty)
        }
    }
}

#[snippet("btree")]
enum BTree<T>
where
    T: Ord + Copy
{
    Leaf(Node<T>),
    Empty,
}

#[snippet("btree")]
impl<T> BTree<T>
where
    T: Ord + Copy
{
    fn new() -> Self {
        BTree::Empty
    }

    fn insert(&mut self, value: T) -> bool {
        match self {
            &mut BTree::Leaf(ref mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less    => { node.right.insert(value); }
                    Ordering::Greater => { node.left.insert(value); }
                    Ordering::Equal   => return false,
                }
            },
            &mut BTree::Empty => {
                *self = BTree::Leaf(Node::new(value))
            },
        };
        true
    }

    fn is_empty(&self) -> bool {
        match self {
            &BTree::Empty    => true,
            &BTree::Leaf(..) => false,
        }
    }

    fn find(&self, value: T) -> bool {
        match self {
            &BTree::Leaf(ref node) => {
                match value.cmp(&node.value) {
                    Ordering::Less    => { node.right.find(value) },
                    Ordering::Greater => { node.left.find(value) },
                    Ordering::Equal   => return true,
                }
            },
            &BTree::Empty => false,
        }
    }

    fn first(&self) -> Option<T> {
        match self {
            &BTree::Leaf(ref node) => {
                let right = &*node.right;
                match right.first() {
                    None => match right {
                        BTree::Leaf(ref node) => Some(node.value),
                        BTree::Empty => None,
                    },
                    Some(_) => unreachable!(),
                }
            },
            &BTree::Empty => None,
        }
    }

    fn last(&self) -> Option<T> {
        match self {
            &BTree::Leaf(ref node) => {
                let left = &*node.left;
                match left.first() {
                    None => match left {
                        BTree::Leaf(ref node) => Some(node.value),
                        BTree::Empty          => None,
                    },
                    Some(_) => unreachable!(),
                }
            },
            &BTree::Empty => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_u32() {
        let mut tree = BTree::new();
        assert!(tree.is_empty());
        tree.insert(8);
        tree.insert(1);
        tree.insert(7);
        tree.insert(4);
        tree.insert(9);
        assert!(!tree.is_empty());
        assert!(tree.find(8));
        assert!(tree.find(4));
        assert!(!tree.find(2));
    }

    #[test]
    fn test_binary_tree_first() {
        let mut tree = BTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.first(), None);
        tree.insert(8);
        tree.insert(1);
        tree.insert(7);
        tree.insert(4);
        tree.insert(9);
        assert_eq!(tree.first(), Some(1));
    }

    #[test]
    fn test_binary_tree_last() {
        let mut tree = BTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.last(), None);
        tree.insert(8);
        tree.insert(1);
        tree.insert(7);
        tree.insert(4);
        tree.insert(9);
        assert_eq!(tree.last(), Some(9));
    }
}