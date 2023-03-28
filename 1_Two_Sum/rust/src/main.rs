use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a new HashMap
        let mut map = HashMap::new();
        // Iterate over the vector of numbers
        for (i, num) in nums.iter().enumerate() {
            // Find the complement of the current value
            let complement = target - num;
            // If the complement exists in the HashMap, return the current index and the index of the complement
            if map.contains_key(&complement) {
                // Return a vector of the current index and the index of the complement
                return vec![map[&complement], i as i32];
            }
            // Add the current value and its index to the HashMap
            map.insert(num, i as i32);
        }
        // If no two numbers add up to the target, return an empty vector
        vec![]
    }
}

fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    assert_eq!(Solution::two_sum(nums1, target1), vec![0, 1]);

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    assert_eq!(Solution::two_sum(nums2, target2), vec![1, 2]);

    let nums3 = vec![3, 3];
    let target3 = 6;
    assert_eq!(Solution::two_sum(nums3, target3), vec![0, 1]);
}
