impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;
        
        while i < nums.len() {
            let num = nums[i];
            for x in i + 1..nums.len() {
                if num == nums[x] {
                    result += 1;
                }
            }
            i += 1;
        }
        
        result
    }
}