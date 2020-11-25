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
    // Time O(N) Space O(N)
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths: Vec<String> = vec![];
        match root {
            None => paths,
            Some(r) => {
                let mut node_stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
                let mut path_stack: Vec<String> = vec![];
                path_stack.push(r.borrow().val.to_string());
                node_stack.push(Some(r));
                while let (Some(Some(node)), Some(path)) = (node_stack.pop(), path_stack.pop()) {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        paths.push(path);
                    } else {
                        if node.borrow().left.is_some() {
                            node_stack.push(node.borrow().left.clone());
                            let new_path = path.clone()
                                + "->"
                                + node
                                    .borrow()
                                    .left
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .val
                                    .to_string()
                                    .as_str();
                            path_stack.push(new_path);
                        }
                        if node.borrow().right.is_some() {
                            
                            node_stack.push(node.borrow().right.clone());
                            let new_path = path
                                + "->"
                                + node
                                    .borrow()
                                    .right
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .val
                                    .to_string()
                                    .as_str();
                            path_stack.push(new_path);
                        }
                    }
                }

                paths
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_257() {
        assert_eq!(
            Solution::binary_tree_paths(to_tree(vec![Some(1), Some(2), Some(3), None, Some(5)])),
            vec!["1->3".to_string(), "1->2->5".to_string()]
        );
    }
}
