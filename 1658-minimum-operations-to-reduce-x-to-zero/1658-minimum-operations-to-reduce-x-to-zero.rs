impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        
        // [a, b, ..., ... ,... y, z]의 배열이 존재하고 target은 x일 때
        // 가운데 배열의 합은 (sum - x). 가운데 배열의 길이가 최대로 길어야 가장 적은 경우의 수로 0을 만들 수 있음. 

        let n = nums.len() as i32;
        let target = nums.iter().sum::<i32>() - x;
      
        if target == 0 { // 배열 전체의 합과 target이 같으므로 배열의 길이를 리턴
            return n;
        }
        
        let (mut max_len, mut cur_sum, mut left) = (0, 0, 0);

        println!("target: {}", target);
        
        for right in 0..n as usize {
            cur_sum += nums[right];

            // 슬라이딩 윈도우를 사용해 현재 부분배열을 조정함.
            // 우리가 찾아야 하는 가운데 배열을 찾는 과정
            // cur_sum이 target보다 크다면 찾는 배열이 많은 요소를 포함하고 있음.
            // 왼쪽 끝의 요소를 제거하고 left를 증가시킴.
            while left <= right as i32 && cur_sum > target {
                println!("cur_sum: {}", cur_sum);
                println!("left: {}", left);
                println!("right: {}", right);
                // 0, 3 -> [1, 1, 4, 2] -> 8
                // 1, 3 -> [1, 4, 2] -> 7
                // 2, 3 -> [4, 2] -> 9
                cur_sum -= nums[left as usize];
                left += 1;
            }
            // 위 while문이 종료되었다는 것은 cur_sum이 target보다 작거나 같은값을 갖게 됨.
            // 즉,목표로 하는 배열의 합이 목표 값에 근접하거나 작아짐


            if cur_sum == target {
                // 현재 배열의 총합과 타겟이 같으면 길이가 가장 큰 배열의 크기를 저장해야 함.
                max_len = std::cmp::max(max_len, right as i32 - left + 1);
            }
        }
        
        // 전체 배열의 길이에서 가운데 배열의 길이를 뺀 값을 리턴.
        if max_len != 0 { n - max_len } else { -1 }
    }
}