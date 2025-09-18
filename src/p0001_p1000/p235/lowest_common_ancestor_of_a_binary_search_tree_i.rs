use std::{cell::RefCell, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Binary Search Tree, Depth-First Search, Tree>
// Time: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.
// Space: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let path_p_arr = Self::build_path_arr(root.clone(), p);
        let path_q_arr = Self::build_path_arr(root, q);
        for i in path_p_arr.iter().rev() {
            if path_q_arr.contains(i) {
                return Some(Rc::clone(i));
            }
        }
        None
    }

    fn build_path_arr(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Rc<RefCell<TreeNode>>> {
        let mut path = Vec::new();
        let target_val = target.as_ref().unwrap().borrow().val;
        let mut cur = root;
        while let Some(node) = cur {
            path.push(Rc::clone(&node));
            if node.borrow().val == target_val {
                break;
            }
            cur = if target_val < node.borrow().val {
                node.borrow().left.clone()
            } else {
                node.borrow().right.clone()
            };
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::implementation::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            6,
            2,
            8,
            0,
            4,
            7,
            9,
            i32::MIN,
            i32::MIN,
            3,
            5,
        ]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 6);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            6,
            2,
            8,
            0,
            4,
            7,
            9,
            i32::MIN,
            i32::MIN,
            3,
            5,
        ]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_case_3() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![2, 1]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
