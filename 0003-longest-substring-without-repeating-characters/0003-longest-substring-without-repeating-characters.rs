impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length: usize = 0;
        let mut start: usize = 0;
        // ASCII 문자셋은 0부터 127까지 정수 값으로 표현되므로 아래 배열을 
        let mut char_indices: [usize; 128] = [0; 128]; // ASCII 문자셋을 가정

        for (end, ch) in s.chars().enumerate() {
            start = start.max(char_indices[ch as usize]); // 현재 문자의 마지막 등장 위치를 확인하고 start 업데이트
            max_length = max_length.max(end - start + 1); // 현재까지의 부분 문자열 길이 중 최대값 업데이트
            char_indices[ch as usize] = end + 1; // 현재 문자의 등장 위치 업데이트
        }

        max_length as i32
    }

}