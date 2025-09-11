use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// DFS

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) =>
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone()),
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
        let p = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2, 3]);
        let q = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2, 3]);
        let result = Solution::is_same_tree(p, q);
        assert!(result);
    }

    #[test]
    fn test_case_2() {
        let p = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2]);
        let q = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, i32::MIN, 2]);
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }

    #[test]
    fn test_case_3() {
        let p = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 2, 1]);
        let q = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, 1, 2]);
        let result = Solution::is_same_tree(p, q);
        assert!(!result);
    }
}
