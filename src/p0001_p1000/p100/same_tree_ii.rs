use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(w), where w is the maximum width of the tree (maximum number of nodes at any level)

// BFS

pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut deque = VecDeque::new();
        deque.push_back((p, q));
        while !deque.is_empty() {
            let (p, q) = deque.pop_front().unwrap();
            match (p, q) {
                (None, None) => continue,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(p), Some(q)) => {
                    if p.borrow().val != q.borrow().val {
                        return false;
                    }
                    deque.push_back((p.borrow().left.clone(), q.borrow().left.clone()));
                    deque.push_back((p.borrow().right.clone(), q.borrow().right.clone()));
                },
            }
        }
        true
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
