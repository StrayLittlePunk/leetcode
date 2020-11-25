#![allow(unused)]
pub struct Solution {}

use std::collections::VecDeque;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut degrees = vec![0; n as usize];
        let mut queue = VecDeque::new();
        let mut graph = vec![Vec::new(); n as usize];
        let mut num_vertexes = n;

        edges.iter().for_each(|edge| {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        });

        for edge in edges {
            let first_vertex = edge[0] as usize;
            let second_vertex = edge[1] as usize;
            degrees[first_vertex] = degrees[first_vertex] + 1;
            degrees[second_vertex] = degrees[second_vertex] + 1;
        }

        degrees.iter().enumerate().for_each(|(idx, degree)| {
            if *degree == 1 {
                queue.push_back(idx)
            }
        });

        while num_vertexes > 2 {
            for _ in 0..queue.len() {
                let leaf_to_remove = queue.pop_front().unwrap() as usize;
                num_vertexes -= 1;

                degrees[leaf_to_remove] = degrees[leaf_to_remove] - 1;

                for adjacent_vertex in &graph[leaf_to_remove] {
                    degrees[*adjacent_vertex as usize] = degrees[*adjacent_vertex as usize] - 1;

                    if degrees[*adjacent_vertex as usize] == 1 {
                        queue.push_back(*adjacent_vertex as usize);
                    }
                }
            }
        }

        return queue.iter().map(|val| *val as i32).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_310() {
        // code here
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
        assert_eq!(
            Solution::find_min_height_trees(2, vec![vec![0, 1]]),
            vec![0, 1]
        );
    }
}
