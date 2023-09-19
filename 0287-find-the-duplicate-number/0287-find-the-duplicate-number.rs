use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // HashSet을 이용한 풀이
        let mut seen = HashSet::new();
        for &num in nums.iter() {
            if seen.contains(&num) {
                return num;
            }
            seen.insert(num);
        }
        -1  // Just to satisfy the compiler, this should never be reached
    }
}