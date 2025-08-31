use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::TreeNode;

/// Self-defined traits
pub trait BinaryTreeExt {
    fn from_vec(values: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

impl BinaryTreeExt for Option<Rc<RefCell<TreeNode>>> {
    fn from_vec(values: Vec<i32>) -> Self {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(values[0])));
        let mut deque: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        deque.push_back(Rc::clone(&root));
        let mut idx = 1;
        let n = values.len();
        while idx < n && !deque.is_empty() {
            if let Some(node) = deque.pop_front() {
                let mut node = node.borrow_mut();
                if values[idx] != i32::MIN {
                    let node_left = Rc::new(RefCell::new(TreeNode::new(values[idx])));
                    node.left = Some(Rc::clone(&node_left));
                    deque.push_back(node_left);
                }
                idx += 1;
                if values[idx] != i32::MIN {
                    let node_right = Rc::new(RefCell::new(TreeNode::new(values[idx])));
                    node.right = Some(Rc::clone(&node_right));
                    deque.push_back(node_right);
                }
                idx += 1;
            }
        }
        Some(root)
    }
}
