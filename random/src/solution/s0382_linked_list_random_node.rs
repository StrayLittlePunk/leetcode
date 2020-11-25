#![allow(unused)]
use crate::util::linked_list::{to_list, ListNode};
use rand::{rngs::ThreadRng, thread_rng, Rng};

struct ListIterator<'a> {
    head: &'a Option<Box<ListNode>>,
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.as_ref().and_then(|head| {
            self.head = &head.next;
            head.next.as_ref()
        })
    }
}

struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let list_iterator = ListIterator { head: &self.head };

        list_iterator
            .into_iter()
            .fold(
                (1, self.head.as_ref().unwrap()),
                |(count, x), cur| match rng.gen_range(0, count + 1) {
                    0 => (count + 1, cur),
                    _ => (count + 1, x),
                },
            )
            .1
            .val
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_382() {
        // let obj = Solution::new(to_list(vec![1, 2, 3]));
        // assert!(obj.get_random() == 1 || obj.get_random() == 2 || obj.get_random() == 3);
    }
}
