struct Solution;

impl Solution {
    //sorting two vectors of chars, and if both equal then is a anagram.
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut vec1: Vec<char> = s.chars().collect();
        let mut vec2: Vec<char> = t.chars().collect();
        vec1.sort();
        vec2.sort();
        if vec1 == vec2 {
            return true;
        } 
        false 
    }
}

fn main() {
    let s1 = String::from("anagram");
    let t1 = String::from("nagaram");
    let s2 = String::from("rat");
    let t2 = String::from("car");
    println!("{}", Solution::is_anagram(s1, t1));
    println!("{}", Solution::is_anagram(s2, t2));
}

/*
Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
 */
