// leetcode: Maximum Depth of Binary Tree

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            1 + std::cmp::max(left_depth, right_depth)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_104() {
        let mut tree = TreeNode::new(3);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        tree.right.as_mut().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(15))));
        tree.right.as_mut().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(max_depth(Some(Rc::new(RefCell::new(tree)))), 3);
    }
}
