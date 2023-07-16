#![warn(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items)]

/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 *
 * Note:
 *
 *
 * 	The number of elements initialized in nums1 and nums2 are m and n respectively.
 * 	You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
 *
 *
 * Example:
 *
 *
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * Output: [1,2,2,3,5,6]
 *
 *
 */

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

fn main() {}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0088_example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        let result = vec![1, 2, 2, 3, 5, 6];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, result);
    }

    #[test]
    fn test_0088_example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        let result = vec![1];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, result);
    }

    #[test]
    fn test_0088_example_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        let result = vec![1];

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, result);
    }
}
