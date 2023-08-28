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
        let len = s.len();
        let mut array1: [i32; 26] = [0;26];
        let mut array2: [i32; 26] = [0;26];
        let mut index1;
        let mut index2; 
        for i in 0..len {
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
}

fn main() {
    let s1 = String::from("anagram");
    let t1 = String::from("nagaram");
    let s2 = String::from("rat");
    let t2 = String::from("car");
    println!("{}", Solution::is_anagram2(s1, t1));
    println!("{}", Solution::is_anagram2(s2, t2));
}

/*
Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
 */
