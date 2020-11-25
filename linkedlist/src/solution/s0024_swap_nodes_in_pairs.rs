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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode { val: 0, next: head });
        let mut tail = &mut root;
        loop {
            let mut front = if tail.next.is_none() {
                break;
            } else {
                tail.next.take().unwrap()
            };
            let mut back = if front.next.is_none() {
                tail.next = Some(front);
                break;
            } else {
                front.next.unwrap()
            };
            front.next = back.next.take();
            back.next = Some(front);
            tail.next = Some(back);
            tail = tail.next.as_mut().unwrap();
            tail = tail.next.as_mut().unwrap();
        }
        root.next
    }
    pub fn swap_pairs_r(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut firstnode = head;
        let mut secondnode = firstnode.as_mut().unwrap().next.take();

        firstnode.as_mut().unwrap().next =
            Solution::swap_pairs_r(secondnode.as_mut().unwrap().next.take());
        secondnode.as_mut().unwrap().next = firstnode;
        secondnode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(
            Solution::swap_pairs_r(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
    }
}
