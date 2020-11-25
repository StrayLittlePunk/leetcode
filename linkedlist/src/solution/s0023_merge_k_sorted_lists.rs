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
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl Solution {
    // Time 128 ms 3.3MB
    pub fn merge_k_lists(mut list: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // cause index error out of bound , because index still increase after remove item
        // for i in 0..list.len() {
        //   if list[i].is_none() {
        //     list.remove(i);
        //   }
        // }
        //  check stackoverflow get answer to retain() method to solve the problem
        // remove empty vector
        list.retain(|x| x.is_some());
        if list.is_empty() {
            return None;
        } else if list.len() < 2 {
            return list.pop().unwrap();
        }

        Self::merge_two_lists(
            Self::merge_two_lists(list.pop().unwrap(), list.pop().unwrap()),
            Self::merge_k_lists(list),
        )
    }

    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut p = &mut result;

        while l1.is_some() && l2.is_some() {
            let deref_val = |node: &Box<ListNode>| (*node).val;
            let digit1 = l1.as_ref().map_or(0, deref_val);
            let digit2 = l2.as_ref().map_or(0, deref_val);
            if digit1 <= digit2 {
                let tmp = l1.as_mut().unwrap().next.take();
                p.next = l1.take();
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.next = l2.take();
                l2 = tmp;
            }
            p = p.next.as_mut().unwrap();
        }
        // 如果l1 是None，则把l2 剩余节点接到结果上，反之亦然，不用考虑两个都为None情况
        if l1.is_none() {
            p.next = l2.take();
        } else {
            p.next = l1.take();
        }
        result.next
    }
      
    /* comparison process by priority queue. */
    // time 4ms 3.1mb
     #[allow(dead_code)]
    pub fn merge_k_lists_best(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for mut node in lists {
            if node.is_some() {
                heap.push(node.take()?)
            }
        }
        let mut head = heap.pop()?;
        let mut pointer = &mut head;
        while !heap.is_empty() {
            if pointer.next.is_some() {
                heap.push(pointer.next.take()?);
            }
            pointer.next = Some(heap.pop()?);
            pointer = pointer.next.as_mut()?;
        }
        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6])
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
