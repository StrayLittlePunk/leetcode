#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1)
    pub fn number_to_words(mut num: i32) -> String {
        match num {
            0 => "Zero".to_owned(),
            1 => "One".to_owned(),
            2 => "Two".to_owned(),
            3 => "Three".to_owned(),
            4 => "Four".to_owned(),
            5 => "Five".to_owned(),
            6 => "Six".to_owned(),
            7 => "Seven".to_owned(),
            8 => "Eight".to_owned(),
            9 => "Nine".to_owned(),
            10 => "Ten".to_owned(),
            11 => "Eleven".to_owned(),
            12 => "Twelve".to_owned(),
            13 => "Thirteen".to_owned(),
            14 => "Fourteen".to_owned(),
            15 => "Fifteen".to_owned(),
            16 => "Sixteen".to_owned(),
            17 => "Seventeen".to_owned(),
            18 => "Eighteen".to_owned(),
            19 => "Nineteen".to_owned(),
            20 => "Twenty".to_owned(),
            30 => "Thirty".to_owned(),
            40 => "Forty".to_owned(),
            50 => "Fifty".to_owned(),
            60 => "Sixty".to_owned(),
            70 => "Seventy".to_owned(),
            80 => "Eighty".to_owned(),
            90 => "Ninety".to_owned(),
            100 => "One Hundred".to_owned(),
            x @ 20..=99 => format!(
                "{} {}",
                Self::number_to_words((x / 10) * 10),
                Self::number_to_words(x % 10)
            ),
            x @ 100..=999 => Self::magnitude("Hundred", x, 100),
            x @ 1000..=999_999 => Self::magnitude("Thousand", x, 1000),
            x @ 1_000_000..=999_999_999 => Self::magnitude("Million", x, 1_000_000),
            x @ 1_000_000_000..=std::i32::MAX => Self::magnitude("Billion", x, 1_000_000_000),
            _ => unimplemented!(),
        }
    }

    fn magnitude(name: &str, number: i32, magnitude: i32) -> String {
        let remainder = number % magnitude;
        if remainder == 0 {
            format!("{} {}", Self::number_to_words(number / magnitude), name)
        } else {
            format!(
                "{} {} {}",
                Self::number_to_words(number / magnitude),
                name,
                Self::number_to_words(remainder)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_273() {
        assert_eq!(
            Solution::number_to_words(123),
            "One Hundred Twenty Three".to_string()
        );
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
        assert_eq!(Solution::number_to_words(1234567891), "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_string());
    }
}
