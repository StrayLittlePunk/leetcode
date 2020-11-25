#![allow(unused)]
pub struct Solution {}

use std::collections::{HashMap, VecDeque};
impl Solution {
    // Time O(V + E) Space O(V + E)
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // For the graph to be a valid tree, it must have exactly n - 1 edges
        if n - 1 != edges.len() as i32 {
            return false;
        }

        let mut adj_list = vec![vec![]; n as usize];
        for edge in edges {
            adj_list[edge[0] as usize].push(edge[1]);
            adj_list[edge[1] as usize].push(edge[0]);
        }

        let mut parent: HashMap<i32, i32> = HashMap::new();
        parent.insert(0, -1);
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(0);

        while let Some(node) = queue.pop_front() {
            for neighbor in adj_list[node as usize].iter() {
                if *neighbor as i32 == *parent.get(&node).unwrap() {
                    continue;
                }

                if parent.contains_key(neighbor) {
                    return false;
                }

                queue.push_back(*neighbor);
                parent.insert(*neighbor, node);
            }
        }

        parent.len() as i32 == n
    }
    // Time O(N) Space O(n)
    pub fn valid_tree_union_find(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // For the graph to be a valid tree, it must have exactly n - 1 edges
        if n - 1 != edges.len() as i32 {
            return false;
        }

        // Initialization  Union-Find Data Structure 
        // parent ids
        let mut parent = vec![];
        // weight each node 
        let mut sz = vec![1; n as usize];
        // connected count
        let mut count = n;
        for i in 0..n as usize {
            parent.push(i as i32);
        }

        // The find method, without any optimizations. It traces up the parent
        // links until it finds the root node for A, and returns that root.
        fn find(mut a: i32, parent: &mut Vec<i32>) -> i32 {
            let ori = a as usize;
            while parent[a as usize] != a {
                a = parent[a as usize];
            }
            parent[ori] = a;
            a
        }
        // The union method, It returns True if a
        // merge happened, False if otherwise.
        fn union(a: i32, b: i32, parent: &mut Vec<i32>, count: &mut i32, sz: &mut Vec<usize>) -> bool {
            // find the roots for a and b
            let root_a = find(a, parent);
            let root_b = find(b, parent);
            // check if a and b are already in the same set.
            if root_a == root_b {
                return false;
            }
            // merge the sets containing a and b

            if sz[root_a as usize] < sz[root_b as usize] {
                parent[root_a as usize] = root_b;
                sz[root_b as usize] += sz[root_a as usize];
            } else {
                parent[root_b as usize] = root_a;
                sz[root_a as usize] += sz[root_b as usize];
            }

            *count -= 1;
            return true;
        }

         // Add each edge. Check if a merge happened, because if it
        // didn't, there must be a cycle.
        for edge in edges {
            if !union(edge[0], edge[1], &mut parent, &mut count, &mut sz){
              return false;
            }
        }

        count == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_261() {
        // code here
        assert_eq!(
            Solution::valid_tree_union_find(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]),
            true
        );
        assert_eq!(
            Solution::valid_tree_union_find(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3]]),
            false
        );
        assert_eq!(
            Solution::valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]),
            true
        );
        assert_eq!(
            Solution::valid_tree(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3]]),
            false
        );
    }
}
