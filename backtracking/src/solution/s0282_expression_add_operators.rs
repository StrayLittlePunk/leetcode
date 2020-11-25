#![allow(unused)]
pub struct Solution {}

// facebook interview
impl Solution {
    pub fn add_operators(a: String, target: i32) -> Vec<String> {
        fn helper(
            ans: &mut Vec<String>,
            path: &mut [u8],
            mut path_index: i32,
            chars: &[u8],
            char_index: i32,
            left: i64,
            cur: i64,
            target: i32,
        ) {
            if char_index == chars.len() as i32 {
                if left + cur == target as i64 {
                    ans.push(
                        std::str::from_utf8(&path[0..path_index as usize])
                            .unwrap()
                            .to_owned(),
                    );
                }
                return;
            }
            let mut n = 0i64;
            let sign_index = path_index;
            path_index += 1;
            for i in char_index..chars.len() as i32 {
                path[path_index as usize] = chars[i as usize];
                path_index += 1;
                n = n * 10 + (chars[i as usize] - b'0') as i64; //add a new digit
                path[sign_index as usize] = b'+';
                helper(ans, path, path_index, chars, i + 1, left + cur, n, target);
                path[sign_index as usize] = b'-';
                helper(ans, path, path_index, chars, i + 1, left + cur, -n, target);
                path[sign_index as usize] = b'*';
                helper(ans, path, path_index, chars, i + 1, left, cur * n, target);
                if n == 0 {
                    break;
                }
            }
        }
        let n = a.len();
        let v = a.as_bytes();
        let mut path = vec![0; n + n];
        let mut ans = vec![];
        let mut x = 0i64;
        for i in 0..n {
            x = x * 10 + (v[i] - b'0') as i64;
            path[i] = v[i];
            helper(
                &mut ans,
                &mut path,
                (i + 1) as i32,
                v,
                (i + 1) as i32,
                0,
                x,
                target,
            );
            if x == 0 {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_282() {
        assert_eq!(
            Solution::add_operators("123".to_owned(), 6),
            ["1+2+3".to_owned(), "1*2*3".to_owned(),]
        );
        assert_eq!(
            Solution::add_operators("232".to_owned(), 8),
            [
            "2+3*2".to_owned(),
            "2*3+2".to_owned(), 
            ]
        );
        assert_eq!(
            Solution::add_operators("105".to_owned(), 5),
            ["1*0+5".to_owned(), "10-5".to_owned(),]
        );
        assert_eq!(
            Solution::add_operators("00".to_owned(), 0),
            ["0+0".to_owned(), "0-0".to_owned(), "0*0".to_owned(),]
        );
    }
}
