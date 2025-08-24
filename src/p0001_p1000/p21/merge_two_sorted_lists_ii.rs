use crate::structs::singly_linked_list::ListNode;

// <Linked List, Recursion>
// Time: O(n + m)
// Space: O(1)

// Iteration

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = &mut list1;
        while list2.is_some() {
            if cur.is_none() || list2.as_ref()?.val < cur.as_ref()?.val {
                std::mem::swap(cur, &mut list2);
            }
            cur = &mut cur.as_mut()?.next;
        }
        list1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let list1 = Option::<Box<ListNode>>::from_vec(vec![1, 2, 4]);
        let list2 = Option::<Box<ListNode>>::from_vec(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let list1 = Option::<Box<ListNode>>::from_vec(Vec::new());
        let list2 = Option::<Box<ListNode>>::from_vec(Vec::new());
        let result = Solution::merge_two_lists(list1, list2);
        let expected = Option::<Box<ListNode>>::from_vec(Vec::new());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let list1 = Option::<Box<ListNode>>::from_vec(Vec::new());
        let list2 = Option::<Box<ListNode>>::from_vec(vec![0]);
        let result = Solution::merge_two_lists(list1, list2);
        let expected = Option::<Box<ListNode>>::from_vec(vec![0]);
        assert_eq!(result, expected);
    }
}
