use crate::structs::singly_linked_list::ListNode;

// <Linked List, Recursion>
// Time: O(n + m)
// Space: O(1)

// Recursion

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(node1), Some(node2)) =>
                if node1.val >= node2.val {
                    Some(Box::new(ListNode {
                        val: node2.val,
                        next: Self::merge_two_lists(Some(node1), node2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: node1.val,
                        next: Self::merge_two_lists(node1.next, Some(node2)),
                    }))
                },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let result = Solution::merge_two_lists(list1, list2);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(4))),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let list1 = None;
        let list2 = None;
        let result = Solution::merge_two_lists(list1, list2);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));
        let result = Solution::merge_two_lists(list1, list2);
        let expected = Some(Box::new(ListNode::new(0)));
        assert_eq!(result, expected);
    }
}
