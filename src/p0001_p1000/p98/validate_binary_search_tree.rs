use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Binary Search Tree, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root, i64::MIN, i64::MAX)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node.as_ref() {
            None => true,
            Some(node) =>
                (node.borrow().val as i64) > min
                    && (node.borrow().val as i64) < max
                    && Self::dfs(node.borrow().left.clone(), min, node.borrow().val as i64)
                    && Self::dfs(node.borrow().right.clone(), node.borrow().val as i64, max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![2, 1, 3]);
        let result = Solution::is_valid_bst(root);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            5,
            1,
            4,
            i32::MIN,
            i32::MIN,
            3,
            6,
        ]);
        let result = Solution::is_valid_bst(root);
        assert!(!result);
    }
}
