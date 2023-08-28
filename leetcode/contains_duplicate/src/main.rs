use std::collections::HashMap;
struct Solution;

impl Solution {
   pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        let len = nums.len();
        for i in 0..len {
            if map.contains_key(&nums[i]){
                return true;
            }
            map.insert(nums[i],nums[i]);
        }
        false
    }
}

fn main() {
    let nums1 = vec![1,2,3,1];
    let nums2 = vec![1,2,3,4];
    let nums3 = vec![1,1,1,3,3,4,3,2,4,2];
    println!("{}",Solution::contains_duplicate(nums1));
    println!("{}",Solution::contains_duplicate(nums2));
    println!("{}",Solution::contains_duplicate(nums3));
}

/*
Example 1:

Input: nums = [1,2,3,1]
Output: true
Example 2:

Input: nums = [1,2,3,4]
Output: false
Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true 
*/
