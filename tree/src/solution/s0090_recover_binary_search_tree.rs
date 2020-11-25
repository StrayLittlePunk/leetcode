// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

#![allow(unused)]
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
// moris traversal https://www.cnblogs.com/AnnieKim/archive/2013/06/15/MorrisTraversal.html
    // Time O(1) best case O(N) worst case , Space O(H) H is tree height
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
      type Node = Option<Rc<RefCell<TreeNode>>>;
        let mut x: Node = None;
        let mut y: Node = None; 
        let mut pred: Node = None;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut r = root.clone();
        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();
            if let Some(node) = r {
                if pred.is_some() && node.borrow().val < pred.as_ref().unwrap().borrow().val {
                    y = Some(node.clone());
                    if x == None {
                        x = pred;
                    } else {
                        break;
                    }
                }

                pred = Some(node.clone());
                r = node.borrow().right.clone();
            }
        }

        match (x, y) {
            (Some(x), Some(y)) => {
                let tmp = x.borrow().val;
                x.borrow_mut().val = y.borrow_mut().val;
                y.borrow_mut().val = tmp;
            }
            (_, _) => {
                return;
            }
        }
    }

    // Time O(N) Space O(N)
    pub fn recover_tree_spacen(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut nums: Vec<i32> = vec![];
        Self::inorder(root, &mut nums);
        let (s1, s2) = Self::find_two_swapped(&nums);
        Self::recover(root, 2, s1, s2);
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        match root {
            Some(node) => {
                Self::inorder(&node.borrow().left.clone(), nums);
                nums.push(node.borrow().val);
                Self::inorder(&node.borrow().right.clone(), nums);
            }
            None => {
                return;
            }
        }
    }

    fn find_two_swapped(nums: &Vec<i32>) -> (i32, i32) {
        let (mut x, mut y) = (-1, -1);

        for i in 0..(nums.len() - 1) {
            if nums[i + 1] < nums[i] {
                y = nums[i + 1];

                if x == -1 {
                    x = nums[i];
                } else {
                    break;
                }
            }
        }

        (x, y)
    }

    fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, mut count: usize, s1: i32, s2: i32) {
        match root {
            Some(node) => {
                let check_val = node.borrow().val;
                if check_val == s1 || check_val == s2 {
                    if check_val == s1 {
                        node.borrow_mut().val = s2;
                    } else {
                        node.borrow_mut().val = s1;
                    }
                    count -= 1;
                    if count == 0 {
                        return;
                    }
                }
                Self::recover(&mut node.borrow().left.clone(), count, s1, s2);
                Self::recover(&mut node.borrow().right.clone(), count, s1, s2);
            }
            None => {
                return;
            }
        }
    }

    pub fn recover_tree_my(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut res: Vec<i32> = vec![];

        let mut r = root.clone();

        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                res.push(node.borrow().val);
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();

            if let Some(node) = r {
                r = node.borrow().right.clone();
            }
        }

        res.sort();

        stack.drain(..);
        r = root.clone();

        let mut i = 0;
        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();
            if let Some(node) = r {
                node.borrow_mut().val = res[i];
                i += 1;
                r = node.borrow().right.clone();
            }
        }
    }

    #[allow(unused)]
    fn build_bst(vector: &Vec<i32>, lo: usize, hi: usize) -> Option<Rc<RefCell<TreeNode>>> {
        // [lo, hi)
        if lo >= hi {
            return None;
        }

        let mid = lo + (hi - lo) / 2;
        let mut root = Rc::new(RefCell::new(TreeNode::new(vector[mid])));

        root.borrow_mut().left = Self::build_bst(vector, lo, mid);
        root.borrow_mut().right = Self::build_bst(vector, mid + 1, hi);

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        let mut root1 = to_tree(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
        Solution::recover_tree(&mut root1);
        assert_eq!(
            root1,
            to_tree(vec![Some(2), Some(1), Some(4), None, None, Some(3)])
        );

        let mut root = to_tree(vec![Some(1), Some(3), None, None, Some(2)]);
        Solution::recover_tree(&mut root);
        assert_eq!(root, to_tree(vec![Some(3), Some(1), None, None, Some(2)]));
    }
}
