use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = vec![];
        let threshold = nums.len() / 3;

        for i in 0..nums.len() {
            let mut entry = map.entry(nums[i]).or_insert(0);
            *entry += 1;

            // 위 두줄을 아래 한줄로 대체 가능
            // *map.entry(nums[i]).or_insert(0) +=1;
        }

        for (key,value) in map {
            if value > threshold {
                result.push(key);
            }
        }

        result
    }
}