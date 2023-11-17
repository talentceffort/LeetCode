use std::cmp::max;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        // 최대 쌍 합이 최소화되도록 주어진 배열의 요소들을 짝을 이루는 것
        // 가장 큰 요소와 가장 작은 요소를 매칭 시켜 주면 됨

        let mut nums = nums; 
        nums.sort();

        let mut i = 0;
        let mut j = nums.len() - 1;

        let mut result = 0;

        while i < j {
            result = max(result, nums[i] + nums[j]);
            i += 1;
            j -= 1;
        }

        result

        
    }
}