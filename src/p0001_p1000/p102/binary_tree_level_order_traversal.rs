use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Tree>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut deque = VecDeque::new();
        deque.push_back(root.unwrap());
        let mut ans = Vec::new();
        while !deque.is_empty() {
            let level_size = deque.len();
            let mut level = Vec::new();
            for _ in 0..level_size {
                let node = deque.pop_front().unwrap();
                level.push(node.borrow().val);
                if let Some(left) = &node.borrow().left {
                    deque.push_back(left.clone());
                }
                if let Some(right) = &node.borrow().right {
                    deque.push_back(right.clone());
                }
            }
            ans.push(level);
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
        let result = Solution::level_order(root);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![1]);
        let result = Solution::level_order(root);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(Vec::new());
        let result = Solution::level_order(root);
        let expected: Vec<Vec<i32>> = Vec::new();
        assert_eq!(result, expected);
    }
}
