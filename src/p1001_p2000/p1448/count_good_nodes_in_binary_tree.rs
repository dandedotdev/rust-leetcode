use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root, i32::MIN, &mut ans);
        ans
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur_max: i32, ans: &mut i32) {
        if let Some(node) = node {
            if node.borrow().val >= cur_max {
                *ans += 1;
            }
            let max = node.borrow().val.max(cur_max);
            Self::dfs(node.borrow().left.clone(), max, ans);
            Self::dfs(node.borrow().right.clone(), max, ans);
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
            1,
            4,
            3,
            i32::MIN,
            1,
            5,
        ]);
        let result = Solution::good_nodes(root);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root =
            <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![3, 3, i32::MIN, 4, 2]);
        let result = Solution::good_nodes(root);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1]);
        let result = Solution::good_nodes(root);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_47() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            -1,
            5,
            -2,
            4,
            4,
            2,
            -2,
            i32::MIN,
            i32::MIN,
            -4,
            i32::MIN,
            -2,
            3,
            i32::MIN,
            -2,
            0,
            i32::MIN,
            -1,
            i32::MIN,
            -3,
            i32::MIN,
            -4,
            -3,
            3,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            3,
            -3,
        ]);
        let result = Solution::good_nodes(root);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
