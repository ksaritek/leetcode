struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max_len = 0;
        let mut map = std::collections::HashMap::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if let Some(j) = map.get(c) {
                // If the character is already in the map,
                // update the start position to the next character of the previous occurence.
                start = std::cmp::max(start, *j + 1);
            }
            // Record the latest position of each character.
            map.insert(c, i);
            // Update the max length if necessary.
            max_len = std::cmp::max(max_len, i - start + 1);
        }

        max_len as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
