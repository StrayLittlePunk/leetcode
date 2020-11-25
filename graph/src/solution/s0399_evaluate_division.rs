#![allow(unused)]
pub struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {

    // Let N be the number of input equations and M be the number of queries.
    //Time Complexity: O(M⋅N)
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        // code here

        let mut graph: HashMap<&String, Vec<(f64, &String)>> = HashMap::new();
        for i in 0..equations.len() {
            (*graph.entry(&equations[i][0]).or_insert(vec![])).push((values[i], &equations[i][1]));
            (*graph.entry(&equations[i][1]).or_insert(vec![]))
                .push((1.0 / values[i], &equations[i][0]));
        }
        queries
            .into_iter()
            .map(|v| {
                let mut seen = HashSet::new();
                let mut q: VecDeque<(f64, &String)> = VecDeque::new();
                if let Some(edges) = graph.get(&v[0]) {
                    for edge in edges {
                        q.push_back(edge.clone());
                    }
                }
                while let Some((val, s)) = q.pop_front() {
                    seen.insert(s.to_owned());
                    if s == &v[1] {
                        return val;
                    } else {
                        for edge in graph.get(&s).unwrap_or(&vec![]) {
                            if !seen.contains(edge.1) {
                                q.push_back((val * edge.0, edge.1));
                            }
                        }
                    }
                }
                -1.0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_399() {
        // code here
        assert_eq!(
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()],
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "e".to_string()],
                    vec!["a".to_string(), "a".to_string()],
                    vec!["x".to_string(), "x".to_string()]
                ]
            ),
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
        );
    }
}
