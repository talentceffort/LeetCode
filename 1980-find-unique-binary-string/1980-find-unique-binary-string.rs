impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        // 칸토어의 대각선 논증
        // 아래는 위키 링크
        // https://ko.wikipedia.org/wiki/%EB%8C%80%EA%B0%81%EC%84%A0_%EB%85%BC%EB%B2%95

        let n = nums.len();
        let mut answer = String::with_capacity(n);
        
        for i in 0..n {
            if nums[i].as_bytes()[i] as char == '1' {
                answer.push('0');
                continue;
            }
            answer.push('1');
        }

        answer
    }
}