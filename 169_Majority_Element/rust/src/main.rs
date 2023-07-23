#![warn(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items)]

/*
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.



Example 1:

Input: nums = [3,2,3]
Output: 3
Example 2:

Input: nums = [2,2,1,1,1,2,2]
Output: 2


Constraints:

n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109


Follow-up: Could you solve the problem in linear time and in O(1) space?
*/

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut maps: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut candidate = nums[0];
        let mut count = 0;

        for num in nums {
            let count = maps.entry(num).or_insert(0);
            *count += 1;
        }

        for (key, value) in maps.iter() {
            if *value > count {
                candidate = *key;
                count = *value;
            }
        }

        candidate
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
