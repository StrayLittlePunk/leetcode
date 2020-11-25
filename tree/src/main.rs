use tree::util::tree::to_tree;
use tree::solution::s0090_recover_binary_search_tree::{Solution};

fn main() {
      let mut root = to_tree(vec![Some(1), Some(3), None, None, Some(2)]);
      Solution::recover_tree(&mut root);
      assert_eq!(root, to_tree(vec![Some(3), Some(1), None, None, Some(2)]));


      let mut root1 = to_tree(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
      Solution::recover_tree(&mut root1);
      assert_eq!(root1, to_tree(vec![Some(2), Some(1), Some(4), None, None, Some(3)]));

}
