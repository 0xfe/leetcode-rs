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

pub fn is_palindrome(
    nodes: Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> (bool, Vec<Option<Rc<RefCell<TreeNode>>>>) {
    if nodes.is_empty() {
        return (true, vec![]);
    }

    let len = nodes.len();

    for i in 0..len / 2 {
        let l = nodes[i].clone();
        let r = nodes[len - i - 1].clone();

        if l.is_none() != r.is_none() {
            return (false, vec![]);
        }

        if let Some(l) = l {
            if let Some(r) = r {
                if l.borrow().val != r.borrow().val {
                    return (false, vec![]);
                }
            }
        }
    }

    let mut new_nodes = vec![];
    for node in nodes.iter().flatten() {
        new_nodes.push(node.borrow().left.clone());
        new_nodes.push(node.borrow().right.clone());
    }

    (true, new_nodes)
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut result = is_palindrome(vec![root]);

    loop {
        if !result.0 {
            return false;
        }

        if result.1.is_empty() {
            break;
        }

        result = is_palindrome(result.1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        tree.left.as_mut().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        tree.left.as_mut().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        tree.right.as_mut().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        tree.right.as_mut().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert!(is_symmetric(Some(Rc::new(RefCell::new(tree)))));
    }
}
