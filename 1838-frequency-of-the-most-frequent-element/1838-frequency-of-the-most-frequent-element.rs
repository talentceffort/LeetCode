use std::cmp;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        // k번 만큼 +1 증가 시킬 수 있음
        // Sliding Window 활용
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut ans = 0;
        let mut curr = 0;

        for right in 0..nums.len() {
            let mut target = nums[right] as usize;
            curr += target;

            // 현재 창의 요소 합계를 조정하여 창의 모든 요소를 동일하게 만들 수 있는지 여부를 확인
            while (right - left + 1) * target - curr > k as usize {
                // 조정은 curr nums[left]를 빼고 left를 증가시켜 수행
                curr -= nums[left] as usize;
                left += 1;
            }
            ans = cmp::max(ans, right - left + 1);
        }

        ans as i32
        
    }
}