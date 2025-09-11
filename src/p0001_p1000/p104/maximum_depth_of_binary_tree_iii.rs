use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(w), where w is the maximum width of the tree (maximum number of nodes at any level)

// BFS

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut deque = VecDeque::new();
        deque.push_back(root.unwrap());
        let mut ans = 0;
        while !deque.is_empty() {
            ans += 1;
            let level_size = deque.len();
            for _ in 0..level_size {
                let node = deque.pop_front().unwrap();
                let node_ref = node.borrow();
                if let Some(left) = &node_ref.left {
                    deque.push_back(left.clone());
                }
                if let Some(right) = &node_ref.right {
                    deque.push_back(right.clone());
                }
            }
        }
        ans
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
        let result = Solution::max_depth(root);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1, i32::MIN, 2]);
        let result = Solution::max_depth(root);
        let expected = 2;
        assert_eq!(result, expected);
    }
}
