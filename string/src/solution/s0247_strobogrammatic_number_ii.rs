#![allow(unused)]
pub struct Solution {}
impl Solution {
    // O(5^(n/2)) O(1)
    pub fn find_strobogrammatic(mut n: i32) -> Vec<String> {
        let mut ans;
        let mut cur;
        if n & 1 == 0 {
            ans = vec!["".to_owned()];
        } else {
            ans = vec!["0".to_owned(), "1".to_owned(), "8".to_owned()];
        }

        if n < 2 {
            return ans;
        }

        while n > 1 {
            cur = std::mem::replace(&mut ans, vec![]);

            for mut s in cur.into_iter() {
                if n > 3 {
                    let mut tmp = s.clone();
                    tmp.insert(0, '0');
                    tmp.push('0');
                    ans.push(tmp);
                }

                let mut t1 = s.clone();
                t1.insert(0, '1');
                t1.push('1');
                ans.push(t1);
                let mut t2 = s.clone();
                t2.insert(0, '8');
                t2.push('8');
                ans.push(t2);
                let mut t3 = s.clone();
                t3.insert(0, '6');
                t3.push('9');
                ans.push(t3);
                s.insert(0, '9');
                s.push('6');
                ans.push(s);
            }

            n -= 2;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_246() {
        assert_eq!(
            Solution::find_strobogrammatic(2),
            vec![
                "11".to_owned(),
                "88".to_owned(),
                "69".to_owned(),
                "96".to_owned()
            ]
        );
    }
}
