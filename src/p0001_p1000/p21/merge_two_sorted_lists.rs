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
        let mut result = ListNode::new(i32::MIN);
        let mut cur_node = &mut result;

        while let (Some(node1), Some(node2)) = (&mut list1, &mut list2) {
            if node1.val >= node2.val {
                cur_node.next = list2;
                list2 = cur_node.next.as_mut().unwrap().next.take();
            } else {
                cur_node.next = list1;
                list1 = cur_node.next.as_mut().unwrap().next.take();
            }
            cur_node = cur_node.next.as_mut().unwrap();
        }

        if list1.is_some() {
            cur_node.next = list1;
        }

        if list2.is_some() {
            cur_node.next = list2;
        }

        result.next
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
