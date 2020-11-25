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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() { return head; }

    let mut to_remove = head.as_ref().unwrap().val - 1;
    let mut dummy = Some(Box::new(ListNode { next: head, val: to_remove, }));
    let mut node = &mut dummy.as_mut().unwrap().next;

    loop {
        match node {
            None => return dummy.unwrap().next,
            Some(n) if n.val == to_remove => *node = n.next.take(),
            Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => to_remove = n.val,
            Some(n) => {
                node = &mut n.next;
                if let Some(n) = node { to_remove = n.val - 1; }
            }
        }
    }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_82() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 2, 3, 3, 4, 4, 5])),
            to_list(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 1, 2, 3])),
            to_list(vec![2, 3])
        );
    }
}
