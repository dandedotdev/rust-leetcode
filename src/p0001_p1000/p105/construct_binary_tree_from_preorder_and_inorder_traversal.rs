use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Array, Binary Tree, Divide and Conquer, Hash Table, Tree>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index_map = inorder
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, idx))
            .collect::<HashMap<_, _>>();
        Self::build(
            &mut preorder.iter(),
            &index_map,
            (0, preorder.len() as isize - 1),
        )
    }

    fn build(
        preorder: &mut std::slice::Iter<i32>,
        index_map: &HashMap<i32, usize>,
        range: (isize, isize), // (pre_start, pre_end)
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if range.0 <= range.1 {
            if let Some(&val) = preorder.next() {
                if let Some(&idx) = index_map.get(&val) {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: Self::build(preorder, index_map, (range.0, idx as isize - 1)),
                        right: Self::build(preorder, index_map, (idx as isize + 1, range.1)),
                    })));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::binary_tree::BinaryTreeExt;

    #[test]
    fn test_case_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let result = Solution::build_tree(preorder, inorder);
        let expected = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![
            3,
            9,
            20,
            i32::MIN,
            i32::MIN,
            15,
            7,
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let result = Solution::build_tree(preorder, inorder);
        let expected = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(vec![-1]);
        assert_eq!(result, expected);
    }
}
