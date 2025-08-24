use crate::structs::singly_linked_list::ListNode;

// <Linked List, Two Pointers>
// Time: O(n)
// Space: O(1)

// Recursion

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end_recr(head, n).0
    }

    fn remove_nth_from_end_recr(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        match head {
            None => (None, 1), // 1 for the last node
            Some(cur) => {
                let (next, count) = Self::remove_nth_from_end_recr(cur.next, n);
                if count == n {
                    (next, count + 1)
                } else {
                    (Some(Box::new(ListNode { val: cur.val, next })), count + 1)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 4, 5]);
        let n = 2;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 2, 3, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1]);
        let n = 1;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(Vec::new());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let head = Option::<Box<ListNode>>::from_vec(vec![1, 2]);
        let n = 1;
        let result = Solution::remove_nth_from_end(head, n);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1]);
        assert_eq!(result, expected);
    }
}
