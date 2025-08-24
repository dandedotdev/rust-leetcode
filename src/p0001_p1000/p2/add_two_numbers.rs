use crate::structs::singly_linked_list::ListNode;

// <Linked List, Math, Recursion>
// Time: O(max(m, n))
// Space: O(max(m, n))

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut dummy = ListNode {
            val: -1,
            next: None,
        };
        let mut cur = &mut dummy;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            if let Some(node) = l1 {
                l1 = node.next;
            }
            if let Some(node) = l2 {
                l2 = node.next;
            }
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let l1 = Option::<Box<ListNode>>::from_vec(vec![2, 4, 3]);
        let l2 = Option::<Box<ListNode>>::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = Option::<Box<ListNode>>::from_vec(vec![7, 0, 8]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let l1 = Option::<Box<ListNode>>::from_vec(vec![0]);
        let l2 = Option::<Box<ListNode>>::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = Option::<Box<ListNode>>::from_vec(vec![0]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let l1 = Option::<Box<ListNode>>::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = Option::<Box<ListNode>>::from_vec(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        let expected = Option::<Box<ListNode>>::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(result, expected);
    }
}
