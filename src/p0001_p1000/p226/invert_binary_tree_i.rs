use std::{cell::RefCell, rc::Rc};

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
        let mut queue = Vec::new();
        queue.push(root.clone().unwrap());
        while !queue.is_empty() {
            if let Some(node) = queue.pop() {
                let TreeNode { left, right, .. } = &mut *node.borrow_mut();
                std::mem::swap(left, right);
                if let Some(left_node) = left.clone() {
                    queue.push(left_node);
                }
                if let Some(right_node) = right.clone() {
                    queue.push(right_node);
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
