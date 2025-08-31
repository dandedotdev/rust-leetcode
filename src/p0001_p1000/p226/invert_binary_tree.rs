use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Recursion>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut deque = VecDeque::new();
        deque.push_back(root.clone().unwrap());
        while !deque.is_empty() {
            if let Some(node) = deque.pop_front() {
                let node_left = node.borrow().left.clone();
                let node_right = node.borrow().right.clone();
                node.borrow_mut().left = node_right;
                node.borrow_mut().right = node_left;
                if let Some(left_node) = node.borrow().left.clone() {
                    deque.push_back(left_node);
                }
                if let Some(right_node) = node.borrow().right.clone() {
                    deque.push_back(right_node);
                }
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root =
            <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![4, 2, 7, 1, 3, 6, 9]);
        let result = Solution::invert_tree(root);
        let expected =
            <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![4, 7, 2, 9, 6, 3, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![2, 1, 3]);
        let result = Solution::invert_tree(root);
        let expected = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![2, 3, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(Vec::new());
        let result = Solution::invert_tree(root);
        let expected = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(Vec::new());
        assert_eq!(result, expected);
    }
}
