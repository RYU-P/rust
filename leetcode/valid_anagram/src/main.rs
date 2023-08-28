use std::{collections::HashMap, hash::Hash};

struct Solution;

impl Solution {
    //sorting two vectors of chars, and if both equal then is a anagram.
    pub fn is_anagram1(s: String, t: String) -> bool {
        let mut vec1: Vec<char> = s.chars().collect();
        let mut vec2: Vec<char> = t.chars().collect();
        vec1.sort();
        vec2.sort();
        if vec1 == vec2 {
            return true;
        } 
        false 
    }

    pub fn is_anagram2(s: String, t: String) -> bool {
        let len1 = s.len();
        let len2 = t.len();
        if len1 != len2{
            return false;
        }
        let mut array1: [i32; 26] = [0;26];
        let mut array2: [i32; 26] = [0;26];
        let mut index1;
        let mut index2; 
        for i in 0..len1 {
            index1 = 
                s.chars()
                .nth(i)
                .map_or(0, |temp|temp as usize - 'a' as usize);
            index2 = 
                t.chars()
                .nth(i)
                .map_or(0, |temp|temp as usize - 'a' as usize);
            array1[index1] += 1;
            array2[index2] += 1;
        }
        if array1 == array2 {
            return true;
        }
        false
    }

    pub fn is_anagram3(s: String, t: String) -> bool {
        let len1 = s.len();
        let len2 = t.len();
        if len1 != len2{
            return false;
        }
        let mut map1: HashMap<char, i32> = HashMap::new();
        let mut map2: HashMap<char, i32> = HashMap::new();
        for i in 0..len1 {
            if let Some(count) = map1.get_mut(&s.chars().nth(i).unwrap()) {
                *count += 1;
            } else {
                map1.insert(s.chars().nth(i).unwrap(),1);
            }
            if let Some(count) = map2.get_mut(&t.chars().nth(i).unwrap()) {
                *count += 1;
            } else {
                map2.insert(t.chars().nth(i).unwrap(),1);
            }
        }
        if map1 == map2 {
            return true;
        }
        false
    }
}

fn main() {
    let s1 = String::from("anagram");
    let t1 = String::from("nagaram");
    let s2 = String::from("a");
    let t2 = String::from("ab");
    println!("{}", Solution::is_anagram3(s1, t1));
    println!("{}", Solution::is_anagram3(s2, t2));
}

/*
Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
 */
