#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(k + M * N) Space O(M * N) k is length of positions
    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {

        if m < 1 || n < 1 {
          return vec![];
        }
        // Initialization  Union-Find Data Structure
        // parent ids
        let mut parent = vec![-1; (n * m) as usize];
        // weight each node
        let mut sz = vec![1; (n * m) as usize];
        // connected count
        let mut count = 0;

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

        let mut ans = vec![];
        // Add each point. Check merge
        for point in positions {
            let idx = point[0] * n + point[1];

            if parent[idx as usize] == -1 {
                parent[idx as usize] = idx;
                count += 1;
            }

            if point[0] - 1 >= 0 && parent[(idx - n) as usize] != -1 {
                union(idx, idx - n, &mut parent, &mut count, &mut sz);
            }
            if point[0] + 1 < m && parent[(idx + n) as usize] != -1 {
                union(idx, idx + n, &mut parent, &mut count, &mut sz);
            }
            if point[1] - 1 >= 0 && parent[(idx - 1) as usize] != -1 {
                union(idx, idx - 1, &mut parent, &mut count, &mut sz);
            }
            if point[1] + 1 < n && parent[(idx + 1) as usize] != -1 {
                union(idx, idx + 1, &mut parent, &mut count, &mut sz);
            }

            ans.push(count);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_305() {
        assert_eq!(
            Solution::num_islands2(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]]),
            vec![1, 1, 2, 3]
        );
        assert_eq!(
            Solution::num_islands2(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![1, 2]]),
            vec![1, 1, 2, 2]
        );
    }
}
