use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Tree>
// Time: O(n)
// Space: O(n)

// BFS

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        let mut deque = VecDeque::new();
        deque.push_back(root.unwrap());
        let mut ans = Vec::new();
        while !deque.is_empty() {
            let level_size = deque.len();
            ans.push(deque.back().unwrap().borrow().val);
            for _ in 0..level_size {
                let node = deque.pop_front().unwrap();
                if let Some(left) = &node.borrow().left {
                    deque.push_back(left.clone());
                }
                if let Some(right) = &node.borrow().right {
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
