use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::structs::binary_tree::TreeNode;

// <Binary Tree, Breadth-First Search, Depth-First Search, Design, String, Tree>
// Time: O(n)
// Space: O(n)

// DPS

pub struct Codec {
    null_marker: &'static str,
    separator: char,
}

impl Codec {
    pub fn new() -> Self {
        Self {
            null_marker: "#",
            separator: ',',
        }
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::default();
        self.dfs_serialize(root, &mut result);
        result
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut tokens: VecDeque<&str> = data.split(self.separator).collect();
        self.dfs_deserialize(&mut tokens)
    }

    fn dfs_serialize(&self, root: Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
        if !result.is_empty() {
            result.push(self.separator);
        }
        match root {
            None => {
                result.push_str(self.null_marker);
            },
            Some(node) => {
                result.push_str(&node.borrow().val.to_string());
                self.dfs_serialize(node.borrow().left.clone(), result);
                self.dfs_serialize(node.borrow().right.clone(), result);
            },
        }
    }

    fn dfs_deserialize(&self, tokens: &mut VecDeque<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        match tokens.pop_front() {
            None => None,
            Some(val_str) => {
                if val_str == self.null_marker {
                    return None;
                }
                let root = Rc::new(RefCell::new(TreeNode::new(val_str.parse().unwrap())));
                root.borrow_mut().left = self.dfs_deserialize(tokens);
                root.borrow_mut().right = self.dfs_deserialize(tokens);
                Some(root)
            },
        }
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
            i32::MIN,
            4,
            5,
        ]);
        let codec = Codec::new();
        let result = codec.serialize(root.clone());
        let expected = "1,2,#,#,3,4,#,#,5,#,#";
        assert_eq!(result, expected);
        let deserialized = codec.deserialize(result);
        assert_eq!(deserialized, root);
    }

    #[test]
    fn test_case_2() {
        let root = <Option<Rc<RefCell<TreeNode>>> as BinaryTreeExt>::from_vec(Vec::new());
        let codec = Codec::new();
        let result = codec.serialize(root.clone());
        let expected = "#";
        assert_eq!(result, expected);
        let deserialized = codec.deserialize(result);
        assert_eq!(deserialized, root);
    }
}
