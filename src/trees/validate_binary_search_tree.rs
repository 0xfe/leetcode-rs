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

pub fn validate_range(
    root: Option<Rc<RefCell<TreeNode>>>,
    left: Option<i32>,
    right: Option<i32>,
) -> bool {
    match root {
        None => true,
        Some(node) => {
            let val = node.borrow().val;
            // Nodes must be less than the right value and greater than the left value

            if let Some(left) = left {
                if val <= left {
                    return false;
                }
            }

            if let Some(right) = right {
                if val >= right {
                    return false;
                }
            }

            validate_range(node.borrow().left.clone(), left, Some(val))
                && validate_range(node.borrow().right.clone(), Some(val), right)
        }
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    validate_range(root, None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_98() {
        let mut tree = TreeNode::new(2);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert!(is_valid_bst(Some(Rc::new(RefCell::new(tree)))));

        let mut tree = TreeNode::new(5);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        tree.right.as_mut().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        tree.right.as_mut().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(6))));
        assert!(!is_valid_bst(Some(Rc::new(RefCell::new(tree)))));
    }
}
