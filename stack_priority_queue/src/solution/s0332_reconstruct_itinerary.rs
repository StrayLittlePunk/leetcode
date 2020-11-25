#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {

      use std::cmp::Reverse;
      use std::collections::{BinaryHeap, HashMap};
      let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();

      for ticket in tickets.iter() {
        graph.entry(&ticket[0]).or_insert_with(BinaryHeap::new).push(Reverse(&ticket[1]));
      }

      let mut ans: Vec<String> = Vec::with_capacity(tickets.len()+1);
      let mut stack: Vec<&str> = vec!["JFK"];
      while let Some(src) = stack.last() {
        if let Some(dsts) = graph.get_mut(src) {
          if !dsts.is_empty() {
            if let Some(dst) = dsts.pop() {
              stack.push(dst.0);
            }
            continue;
          }
        }
        if let Some(last) = stack.pop() {
          ans.push(last.to_string());
        }
      }


      ans.reverse();
      ans

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_218() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["MUC".to_string(), "LHR".to_string()],
                vec!["JFK".to_string(), "MUC".to_string()],
                vec!["SFO".to_string(), "SJC".to_string()],
                vec!["LHR".to_string(), "SFO".to_string()],
            ]),
            vec![
            "JFK".to_string(), "MUC".to_string(), "LHR".to_string(), 
            "SFO".to_string(), "SJC".to_string()
            ]
        );
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["JFK".to_string(), "SFO".to_string()],
                vec!["JFK".to_string(), "ATL".to_string()],
                vec!["SFO".to_string(), "ATL".to_string()],
                vec!["ATL".to_string(), "JFK".to_string()],
                vec!["ATL".to_string(), "SFO".to_string()],
            ]),
            vec![
            "JFK".to_string(), "ATL".to_string(), "JFK".to_string(), 
            "SFO".to_string(), "ATL".to_string(), "SFO".to_string()
            ]
        );
    }
}
