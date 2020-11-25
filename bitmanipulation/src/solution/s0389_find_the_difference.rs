#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(N) O(1) 
    // string是一个用utf-8编码存储的vec<u8> 序列， 而 char 是 unicode 的字符标量值，用 
    // 四个字节表示，注意unicode并没有规定如何字符串的存储方式，所以由字符标量转成在 
    // 磁盘存储的时候要选择编码方式。这里思路首先用u8类型存储u8结果，然后对字符串每个字节 
    // 进行异或赋值 ， a ^= a == 0 
    // 0 ^ 0 = 0
    // 0 ^ 1 = 1
    // 1 ^ 0 = 1
    // 1 ^ 1 = 0
    // 最后得到的结果就是多出来的u8值，然后转为char 返回
    pub fn find_the_difference_ascii(s: String, t: String) -> char {
      let mut ch: u8 = 0 ;

      for c in s.as_bytes() {
        ch ^= c;
      }

      for c in t.as_bytes() {
        ch ^= c;
      }

      char::from(ch)
    }
    // 这里换做字符表示进行异或，可以比对所有合法Unicode字符标量, 上面的只能比对 ascii 类字符
    pub fn find_the_difference(s: String, t: String) -> char {
      let mut ch: u32 = 0 ;

      for c in s.chars() {
        ch ^= c as u32;
      }

      for c in t.chars() {
        ch ^= c as u32;
      }

      match std::char::from_u32(ch) {
        Some(ch) => ch,
        None => '0',
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_389() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcd值".to_string()),
            '值'
        );
        assert_eq!(
            Solution::find_the_difference("".to_string(), "y".to_string()),
            'y'
        );
        assert_eq!(
            Solution::find_the_difference("a".to_string(), "aa".to_string()),
            'a'
        );
        assert_eq!(
            Solution::find_the_difference("ae".to_string(), "aea".to_string()),
            'a'
        );
    }
}
