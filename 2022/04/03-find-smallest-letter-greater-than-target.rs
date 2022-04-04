// https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut left,mut right) = (0,letters.len()-1);
        while left < right {
           let mid = (left+right)/2;
            if letters[mid] <= target {
                left = mid+1;
            }else {
                right = mid;
            }
        }
        if letters[left] > target {
            letters[left]
        }else {
            letters[0]
        }
    }
}