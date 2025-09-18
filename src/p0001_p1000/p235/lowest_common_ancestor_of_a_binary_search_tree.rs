use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Binary Search Tree, Depth-First Search, Tree>
// Time: O(h), where h is the height of the tree, possibly O(n) in the worst case, O(log n) in the best case.
// Space: O(1)

// Iterative

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let mut cur = root;
        while let Some(node) = cur {
            match (p_val.cmp(&node.borrow().val), q_val.cmp(&node.borrow().val)) {
                (Ordering::Less, Ordering::Less) => {
                    cur = node.borrow().left.clone();
                },
                (Ordering::Greater, Ordering::Greater) => {
                    cur = node.borrow().right.clone();
                },
                _ => return Some(Rc::clone(&node)),
            }
        }
        None
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
