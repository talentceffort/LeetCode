impl Solution {
    pub fn decode_at_index(s: String, mut k: i32) -> String {
        let mut length: i64 = 0; // 디코딩된 문자열의 길이 저장
        let mut i = 0; // 압축된 문자열을 반복하는 데 사용
        let bytes = s.as_bytes();

        // length를 구하기 위한 작업
        while length < k as i64 {
            if bytes[i].is_ascii_digit() {
                length *= (bytes[i] as char).to_digit(10).unwrap() as i64;
            } else {
                length += 1;
            }
            i += 1;
        }

        // 위에서 length에 추가한 부분을 효율적으로 제거하기 위해서 역순으로 실행
        for j in (0..i).rev() {
            if bytes[j].is_ascii_digit() {
                length /= (bytes[j] as char).to_digit(10).unwrap() as i64;
                // 나머지로 K 조정. k가 7이고 length가 5인 경우 7의 위치에 있는 문자와 2에 위치한 문자는 같다.
                k = (k as i64 % length) as i32;
            } else {
                if k == 0 || k as i64 == length {
                    return (bytes[j] as char).to_string();
                }
                length -= 1;
            }
        }

        unreachable!()
    }
}