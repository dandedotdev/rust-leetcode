use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Depth-First Search, Dynamic Programming, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MIN;
        Self::dfs(root, &mut ans);
        ans
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let sum_left = Self::dfs(node.borrow().left.clone(), max_sum).max(0);
                let sum_right = Self::dfs(node.borrow().right.clone(), max_sum).max(0);
                *max_sum = (*max_sum).max(node.borrow().val + sum_left + sum_right);
                node.borrow().val + sum_left.max(sum_right)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2, 3]);
        let result = Solution::max_path_sum(root);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            -10,
            9,
            20,
            i32::MIN,
            i32::MIN,
            15,
            7,
        ]);
        let result = Solution::max_path_sum(root);
        let expected = 42;
        assert_eq!(result, expected);
    }
}
