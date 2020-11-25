#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N) Space O(n)
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
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
        fn union(a: i32, b: i32, parent: &mut Vec<i32>, count: &mut i32, sz: &mut Vec<usize>) {
            // find the roots for a and b
            let root_a = find(a, parent);
            let root_b = find(b, parent);
            // check if a and b are already in the same set.
            if root_a == root_b {
                return;
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
        }

         // Add each edge. Check if a merge happened, because if it
        // didn't, there must be a cycle.
        for edge in edges {
            union(edge[0], edge[1], &mut parent, &mut count, &mut sz);
        }

        count
      
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_323() {
        // code here
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );
    }
}
