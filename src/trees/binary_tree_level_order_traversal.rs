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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut result = vec![];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);

    loop {
        let mut level = vec![];
        let mut next_queue = std::collections::VecDeque::new();

        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                level.push(node.borrow().val);
                let left_node = node.borrow().left.clone();
                if left_node.is_some() {
                    next_queue.push_back(node.borrow().left.clone());
                }

                let right_node = node.borrow().right.clone();
                if right_node.is_some() {
                    next_queue.push_back(node.borrow().right.clone());
                }
            }
        }

        result.push(level);

        if next_queue.is_empty() {
            break;
        }
        queue = next_queue;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
        let mut tree = TreeNode::new(3);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        tree.right.as_mut().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(15))));
        tree.right.as_mut().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(
            level_order(Some(Rc::new(RefCell::new(tree)))),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
