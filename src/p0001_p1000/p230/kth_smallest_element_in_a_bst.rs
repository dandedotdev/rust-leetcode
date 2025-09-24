use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Binary Search Tree, Depth-First Search, Tree>
// Time: O(h + k), where h is the height of the tree, k is the kth smallest element
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

// Iterative

pub struct Solution;

impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stk = Vec::new();
        while !stk.is_empty() || root.is_some() {
            while let Some(node) = root {
                stk.push(Rc::clone(&node));
                root = node.borrow().left.clone();
            }
            if let Some(node) = stk.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                root = node.borrow().right.clone();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root =
            <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![3, 1, 4, i32::MIN, 2]);
        let k = 1;
        let result = Solution::kth_smallest(root, k);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            5,
            3,
            6,
            2,
            4,
            i32::MIN,
            i32::MIN,
            1,
        ]);
        let k = 3;
        let result = Solution::kth_smallest(root, k);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
