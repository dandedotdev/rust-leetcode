use std::{cell::RefCell, rc::Rc};

use crate::{
    p0001_p1000::p100::same_tree::Solution as SameTreeSolutions, structs::binary_tree::TreeNode,
};

// <Binary Tree, Depth-First Search, Hash Function, String Matching, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (Some(root), Some(sub_root)) =>
                SameTreeSolutions::is_same_tree(Some(root.clone()), Some(sub_root.clone()))
                    || Self::is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
                    || Self::is_subtree(root.borrow().right.clone(), Some(sub_root)),
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![3, 4, 5, 1, 2]);
        let sub_root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![4, 1, 2]);
        let result = Solution::is_subtree(root, sub_root);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            3,
            4,
            5,
            1,
            2,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            i32::MIN,
            0,
        ]);
        let sub_root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![4, 1, 2]);
        let result = Solution::is_subtree(root, sub_root);
        assert!(!result);
    }
}
