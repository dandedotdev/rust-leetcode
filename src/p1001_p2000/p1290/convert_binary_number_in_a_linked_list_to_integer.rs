use crate::structs::singly_linked_list::ListNode;

// <Linked List, Math>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;

        while let Some(node) = head {
            result = (result << 1) | node.val;
            head = node.next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));
        let result = Solution::get_decimal_value(head);
        let expected = 5;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Some(Box::new(ListNode::new(0)));
        let result = Solution::get_decimal_value(head);
        let expected = 0;

        assert_eq!(result, expected);
    }
}
