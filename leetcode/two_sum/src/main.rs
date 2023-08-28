//two sums
//may assume that there is exactly one solution.
use std::collections::HashMap;
struct Solution;

impl Solution {
    //O(n^2) solution
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j{}
            else if nums[i] + nums[j] == target {
                return vec![i as i32,j as i32];
            }
        }
      } 
      return nums;
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&(target - nums[i])) {
               return vec![i as i32, *map.get(&(&target - nums[i])).unwrap()]; 
           }
           map.insert(nums[i], i as i32);
        }
    return nums;
    }
}
fn main() {
    let nums1 = vec![2,7,11,15];
    let target1= 9;
    let nums2 = vec![3,2,4];
    let target2 = 6;
    let nums3 = vec![3,3];
    let target3 = 6;
    println!("{:?}", Solution::two_sum2(nums1, target1));
    println!("{:?}", Solution::two_sum2(nums2, target2));    
    println!("{:?}", Solution::two_sum2(nums3, target3));
}

/*
Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
 */
