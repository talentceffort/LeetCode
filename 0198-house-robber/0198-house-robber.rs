impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
        }

        *dp.last().unwrap()
    }
}