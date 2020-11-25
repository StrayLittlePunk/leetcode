#![allow(unused)]
pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// reference: https://leetcode.com/problems/insertion-sort-list/discuss/46420/An-easy-and-clear-way-to-sort-(-O(1)-space-)
impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut prev = &mut dummy;

        while let Some(mut node) = head {
            let temp = node.next.take();

            /* Before insert, the prev is at the last node of the sorted list.
            Only the last node's value is larger than the current inserting node
            should we move the temp back to the head*/
            if prev.val > node.val {
                prev = &mut dummy;
            }
            //find the right place to insert
            while prev.next.is_some() && prev.next.as_ref().unwrap().val < node.val {
                prev = prev.next.as_mut().unwrap();
            }
            //insert between pre and pre.next
            node.next = prev.next.take();
            prev.next = Some(node);
            // prev = dummy; // Don't set prev to the head of the list after insert
            head = temp;
        }

        dummy.next
    }
    /* C version insertion_sort
     * int n = a.len();
     * for (int i = 1; i< n; i++) {
     *   for(int j = i; j > 0 && a[j] < a[j-1]; j --){
     *     int tmp = a[j];
     *     a[j] = a[j -1];
     *     a[j-1] = tmp;
     *   }
     * } */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_147() {
        assert_eq!(
            Solution::insertion_sort_list(to_list(vec![4, 2, 1, 3])),
            to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::insertion_sort_list(to_list(vec![-1, 5, 3, 4, 0])),
            to_list(vec![-1, 0, 3, 4, 5])
        );
    }
}
