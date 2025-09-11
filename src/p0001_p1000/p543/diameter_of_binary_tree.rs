use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root, &mut ans);
        ans
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        match node {
            Some(node) => {
                let depth_left = Self::dfs(node.borrow().left.clone(), max_diameter);
                let depth_right = Self::dfs(node.borrow().right.clone(), max_diameter);
                *max_diameter = (*max_diameter).max(depth_left + depth_right);
                depth_left.max(depth_right) + 1
            },
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2, 3, 4, 5]);
        let result = Solution::diameter_of_binary_tree(root);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2]);
        let result = Solution::diameter_of_binary_tree(root);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
