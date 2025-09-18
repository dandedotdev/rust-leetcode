use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::dfs(root, 0, &mut ans);
        ans
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, ans: &mut Vec<i32>) {
        if let Some(node) = node {
            if depth == ans.len() as i32 {
                ans.push(node.borrow().val);
            }
            Self::dfs(node.borrow().right.clone(), depth + 1, ans);
            Self::dfs(node.borrow().left.clone(), depth + 1, ans);
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
            1,
            2,
            3,
            i32::MIN,
            5,
            i32::MIN,
            4,
        ]);
        let result = Solution::right_side_view(root);
        let expected = vec![1, 3, 4];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            1,
            2,
            3,
            4,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            5,
        ]);
        let result = Solution::right_side_view(root);
        let expected = vec![1, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, i32::MIN, 3]);
        let result = Solution::right_side_view(root);
        let expected: Vec<i32> = vec![1, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_4() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(Vec::new());
        let result = Solution::right_side_view(root);
        let expected = Vec::new();
        assert_eq!(result, expected);
    }
}
