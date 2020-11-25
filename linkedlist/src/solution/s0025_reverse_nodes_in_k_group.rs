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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = Self::length(&head) as i32;
        if k < 2 {
            return head;
        }
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut ptr = dummy.as_mut();
        let mut i = 1;
        while let Some(node) = ptr {
            if len >= k && i == 1 {
                len -= k;
                node.next = Self::reverse1(node.next.take(), k);
                i = k;
                ptr = node.next.as_mut();
            } else {
                i -= 1;
                ptr = node.next.as_mut();
            }
        }

        /*         while let Some(node) = ptr {
         *           if len >= k{
         *
         *             // why (node.next, ptr) = Self::reverse1(node.next.take(), k)
         *             // because of Rust can't destructing assignment.
         *             // detail see issues https://github.com/rust-lang/rfcs/issues/372
         *             // let (mut next_node, mut tail) = Self::reverse1(node.next.take(), k);
         *             node.next = next_node;
         *             ptr = tail.as_mut();
         *
         *           } else {
         *             ptr = node.next.as_mut();
         *           }
         *         } */
        dummy.unwrap().next
    }
    // This function can handle the flip of the number of linked lists less than count,
    // for example
    // input:  1->2->none, count = 3,
    // output: 2->1->none
    // then 1->2 can also be flipped
    fn reverse(
        head: Option<Box<ListNode>>,
        acc: Option<Box<ListNode>>,
        count: i32,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            return Self::append(acc, head);
        }
        if let Some(mut node) = head {
            let next = node.next;
            node.next = acc;
            Self::reverse(next, Some(node), count - 1)
        } else {
            acc
        }
    }
    fn reverse1(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        // Reverse k nodes of the given linked list.
        // This function assumes that the list contains
        // atleast k nodes.
        let mut new_head: Option<Box<ListNode>> = None;
        while k > 0 {
            // Keep track of the next node to process in the
            // original list
            let tmp = head.as_mut().unwrap().next.take();
            // Insert the node pointed to by "ptr"
            // at the beginning of the reversed list
            head.as_mut().unwrap().next = new_head;
            new_head = head;
            // Move on to the next node
            head = tmp;
            // Decrement the count of nodes to be reversed by 1
            k -= 1;
        }

        // connect k + 1 node
        let mut ptr = new_head.as_mut().unwrap();
        while ptr.next.is_some() {
            ptr = ptr.next.as_mut().unwrap();
        }
        ptr.next = head;

        // Return the head of the reversed list
        new_head
    }

    fn append(
        mut front: Option<Box<ListNode>>,
        back: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = front.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = back;
                break;
            }
            current = node.next.as_mut();
        }
        front
    }
    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }

    fn reverse_k_group_best(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        // 3 components:
        //  - head: remaining list
        //  - k_group: current accumulating group, targeting k elements
        //  - prev_tail: tail of already processed part of list
        //
        // Basic idea:
        // Take nodes from head, prepend the node to k_group, so after doing it k times,
        // k_group will be the reversed list of size k taken from head.
        // Then we append k_group to prev_tail, and search for the new prev_tail.
        // If there is not enough nodes to form a k_group of size k, that means the current k_group is the last group
        // and its size is smaller than k. In this case, we reverse the k_group again to revert the change, and append
        // it to prev_tail, then return.
        let mut p_head: Option<Box<ListNode>> = None;
        let mut prev_tail = &mut p_head;
        let mut k_group: Option<Box<ListNode>> = None;
        loop {
            for k_group_len in 0..k {
                if let Some(mut node) = head {
                    head = node.next.take();
                    node.next = k_group;
                    k_group = Some(node);
                } else {
                    let mut reverted_k_group: Option<Box<ListNode>> = None;
                    for _ in 0..k_group_len {
                        let mut node = k_group.unwrap();
                        k_group = node.next.take();
                        node.next = reverted_k_group;
                        reverted_k_group = Some(node);
                    }
                    *prev_tail = reverted_k_group;
                    return p_head;
                }
            }
            *prev_tail = k_group;
            for _ in 0..k {
                prev_tail = &mut prev_tail.as_mut().unwrap().next;
            }
            k_group = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 1),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
    }
}
