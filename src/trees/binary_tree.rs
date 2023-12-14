use std::{collections::VecDeque, fmt::Debug};

pub trait TreeNode: Eq + PartialEq + PartialOrd + Debug {}
impl TreeNode for String {}

#[derive(Debug)]
pub struct Node<T: TreeNode> {
    pub value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: TreeNode> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: TreeNode> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

pub struct BinaryTree<T: TreeNode> {
    root: Option<Box<Node<T>>>,
}

impl<T: TreeNode> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        let new_node = Node::new(val);
        let mut current = &mut self.root;

        while let Some(node) = current {
            if node.value < new_node.value {
                current = &mut node.right;
            } else {
                current = &mut node.left;
            }
        }

        *current = Some(Box::new(new_node));
    }

    pub fn search_recursive(&mut self, val: T) -> Option<&Node<T>> {
        let mut current = &mut self.root;

        while let Some(node) = current {
            if node.value == val {
                return Some(node);
            } else if node.value < val {
                current = &mut node.right;
            } else {
                current = &mut node.left;
            }
        }

        None
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: Fn(&Node<T>),
    {
        Self::for_each_internal(&self.root, &f);
    }

    fn for_each_internal(node: &Option<Box<Node<T>>>, f: &impl Fn(&Node<T>)) {
        if let Some(node) = node {
            print!("{:?} ", node);
            f(node);
            Self::for_each_internal(&node.left, f);
            Self::for_each_internal(&node.right, f);
        }
    }

    pub fn bfs_iter(&self) -> BinaryTreeIterator<T> {
        BinaryTreeIterator {
            current: &self.root,
            queue: VecDeque::new(),
            strategy: Strategy::BreadthFirst,
        }
    }

    pub fn dfs_iter(&self) -> BinaryTreeIterator<T> {
        BinaryTreeIterator {
            current: &self.root,
            queue: VecDeque::new(),
            strategy: Strategy::DepthFirst,
        }
    }
}

impl<T: TreeNode> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

enum Strategy {
    BreadthFirst,
    DepthFirst,
}

pub struct BinaryTreeIterator<'a, T: TreeNode> {
    current: &'a Option<Box<Node<T>>>,
    queue: VecDeque<&'a Option<Box<Node<T>>>>,
    strategy: Strategy,
}

impl<'a, T: TreeNode> BinaryTreeIterator<'a, T> {
    fn bfs_next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.current {
            self.queue.push_back(&node.left);
            self.queue.push_back(&node.right);
            self.current = self.queue.pop_front().unwrap_or(&None);
            return Some(&node.value);
        }
        None
    }

    fn dfs_next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.current {
            self.queue.push_front(&node.left);
            self.current = self.queue.pop_front().unwrap_or(&None);
            self.queue.push_front(&node.right);
            return Some(&node.value);
        }
        None
    }
}

impl<'a, T: TreeNode> Iterator for BinaryTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.strategy {
            Strategy::BreadthFirst => self.bfs_next(),
            Strategy::DepthFirst => self.dfs_next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        let mut tree = BinaryTree::new();
        tree.insert("F".to_string());
        tree.insert("B".to_string());
        tree.insert("G".to_string());
        tree.insert("A".to_string());
        tree.insert("D".to_string());
        tree.insert("I".to_string());
        tree.insert("C".to_string());
        tree.insert("E".to_string());
        tree.insert("H".to_string());

        assert_eq!(tree.root.as_ref().unwrap().value, "F");
        assert_eq!(
            tree.root.as_ref().unwrap().left.as_ref().unwrap().value,
            "B"
        );
        assert_eq!(
            tree.root.as_ref().unwrap().right.as_ref().unwrap().value,
            "G"
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            "A"
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            "D"
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            "I"
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            "C"
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            "E"
        );

        assert_eq!(tree.search_recursive("A".to_string()).unwrap().value, "A");
        assert!(tree.search_recursive("XX".to_string()).is_none());
    }

    #[test]
    fn bfs() {
        let mut tree = BinaryTree::new();
        tree.insert("F".to_string());
        tree.insert("B".to_string());
        tree.insert("G".to_string());
        tree.insert("A".to_string());
        tree.insert("D".to_string());
        tree.insert("I".to_string());
        tree.insert("C".to_string());
        tree.insert("E".to_string());
        tree.insert("H".to_string());

        tree.for_each(|n| println!("{}", n.value));

        println!("{:?}", tree.bfs_iter().collect::<Vec<_>>());
    }

    #[test]
    fn dfs() {
        let mut tree = BinaryTree::new();
        tree.insert("F".to_string());
        tree.insert("B".to_string());
        tree.insert("G".to_string());
        tree.insert("A".to_string());
        tree.insert("D".to_string());
        tree.insert("I".to_string());
        tree.insert("C".to_string());
        tree.insert("E".to_string());
        tree.insert("H".to_string());

        tree.for_each(|n| println!("{}", n.value));

        println!("{:?}", tree.bfs_iter().collect::<Vec<_>>());
    }
}
