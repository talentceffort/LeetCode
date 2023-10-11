impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        
        let a = nums.partition_point(|x| x < &target);
        let b = nums.partition_point(|x| x <= &target);
        
        
        if a > nums.len() - 1 || nums[a] != target {
            return vec![-1, -1];
        }
        
        
        vec![a as i32, (b - 1) as i32]
    }
}