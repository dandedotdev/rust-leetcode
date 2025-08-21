use crate::structs::singly_linked_list::ListNode;

// <Linked List, Recursion>
// Time: O(n)
// Space: O(1)

// Note: Recursion

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list_recursive(head, None)
    }

    fn reverse_list_recursive(
        head: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let next = node.next;
            node.next = prev;
            Self::reverse_list_recursive(next, Some(node))
        } else {
            prev
        }
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
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let result = Solution::reverse_list(head);
        let expected = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(1))),
                    })),
                })),
            })),
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        let result = Solution::reverse_list(head);
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let head: Option<Box<ListNode>> = None;
        let result = Solution::reverse_list(head);
        let expected: Option<Box<ListNode>> = None;
        assert_eq!(result, expected);
    }
}
