use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root).is_some()
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match node {
            Some(node) => {
                let depth_left = Self::dfs(node.borrow().left.clone());
                let depth_right = Self::dfs(node.borrow().right.clone());
                if let (Some(left), Some(right)) = (depth_left, depth_right) {
                    if (left - right).abs() <= 1 {
                        Some(left.max(right) + 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            None => Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            3,
            9,
            20,
            i32::MIN,
            i32::MIN,
            15,
            7,
        ]);
        let result = Solution::is_balanced(root);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            1,
            2,
            2,
            3,
            3,
            i32::MIN,
            i32::MIN,
            4,
            4,
        ]);
        let result = Solution::is_balanced(root);
        assert!(!result);
    }
}
