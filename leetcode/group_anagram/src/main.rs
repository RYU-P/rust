
use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for i in 0..strs.len(){
            let mut temp: Vec<char>= strs[i].clone().chars().collect();
            temp.sort();
            map.entry(temp).or_insert(vec![]).push(strs[i].clone());
        }
        let vec: Vec<Vec<String>> = map.values().cloned().collect();
        vec
    }
}

fn main() {
    let strs: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
 
    println!("{:?}", Solution::group_anagram(strs));
}

/*
Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
Example 2:

Input: strs = [""]
Output: [[""]]
Example 3:

Input: strs = ["a"]
Output: [["a"]]
 */