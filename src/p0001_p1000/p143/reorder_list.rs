use std::collections::VecDeque;

use crate::structs::singly_linked_list::ListNode;

// <Two Pointers, Linked List, Recursion, Stack>
// Time: O(n)
// Space: O(n)

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut deque = VecDeque::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            deque.push_back(node);
        }
        let mut ans = ListNode::new(0);
        let (mut cur, mut is_front) = (&mut ans, true);
        while !deque.is_empty() {
            if is_front {
                cur.next = deque.pop_front();
            } else {
                cur.next = deque.pop_back();
            }
            is_front = !is_front;
            cur = cur.next.as_mut().unwrap();
        }
        *head = ans.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_case_2() {
        let mut head = Some(Box::new(ListNode {
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
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }
}
