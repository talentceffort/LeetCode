use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut num_count = HashMap::new();

        let mut result = 0;

        for num in nums.iter() {
            let count = num_count.entry(num).or_insert(0);
            *count += 1;

            if *count > 1 {
                result = *num;
                break
            }
        }

        result
    }
}