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
    #[inline]
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut p = &mut result;

        let mut carry = 0;
        let mut sum = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let deref_val = |node: &Box<ListNode>| (*node).val;
            let digit1 = l1.as_ref().map_or(0, deref_val);
            let digit2 = l2.as_ref().map_or(0, deref_val);
            sum = digit1 + digit2 + carry;
            carry = sum / 10;
            p.next = Some(Box::new(ListNode::new(sum % 10)));
            p = p.next.as_mut().unwrap();

            let deref_next = |node: Box<ListNode>| (*node).next;

            l1 = l1.and_then(deref_next);
            l2 = l2.and_then(deref_next);
            /*             如果使用下列注释语句则会无法处理 None 的解包 ,unwrap直接 panic
             *             Runtime Error Message:
             *             called `Option::unwrap()` on a `None` value (src/libcore/option.rs)
             *             Last executed input:  [5]
             *                                   [5]
             *
             *             l1 = l1.unwrap().next.take();
             *             l2 = l2.unwrap().next.take(); */
        }

        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
    }
}
