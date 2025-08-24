use crate::structs::singly_linked_list::ListNode;

// <Linked List, Math>
// Time: O(n)
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        while let Some(node) = head {
            ans = (ans << 1) | node.val;
            head = node.next;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 0, 1]);
        let result = Solution::get_decimal_value(head);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Option::<Box<ListNode>>::from_vec(vec![0]);
        let result = Solution::get_decimal_value(head);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
