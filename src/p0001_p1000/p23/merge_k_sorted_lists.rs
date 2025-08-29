use crate::{
    p0001_p1000::p21::merge_two_sorted_lists::Solution as MergeTwoSortedListsSolution,
    structs::singly_linked_list::ListNode,
};

// <Divide and Conquer, Heap(Priority Queue), Linked List, Merge Sort>
// Time: O(n log k), where n is the total number of nodes in all lists, k is the number of lists
// Space: O(1)

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        while lists.len() > 1 {
            let mut merged = Vec::new();
            let mut i = 0;
            while i < lists.len() {
                if i + 1 < lists.len() {
                    merged.push(MergeTwoSortedListsSolution::merge_two_lists(
                        lists[i].take(),
                        lists[i + 1].take(),
                    ));
                } else {
                    merged.push(lists[i].take());
                }
                i += 2;
            }
            lists = merged;
        }
        lists[0].take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::singly_linked_list::LinkedListExt;

    #[test]
    fn test_case_1() {
        let lists = vec![
            Option::<Box<ListNode>>::from_vec(vec![1, 4, 5]),
            Option::<Box<ListNode>>::from_vec(vec![1, 3, 4]),
            Option::<Box<ListNode>>::from_vec(vec![2, 6]),
        ];
        let result = Solution::merge_k_lists(lists);
        let expected = Option::<Box<ListNode>>::from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let lists = vec![];
        let result = Solution::merge_k_lists(lists);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        let lists = vec![None];
        let result = Solution::merge_k_lists(lists);
        let expected = None;
        assert_eq!(result, expected);
    }
}
