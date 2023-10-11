impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if let Ok(index) = nums.binary_search(&target) {
            return vec![nums.partition_point(|&i| i < target) as i32, nums.partition_point(|&i| i <= target) as i32 - 1];
        } 
        return vec![-1, -1];
        
    }
}