use std::collections::HashSet;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        println!("{:?}", s.bytes().chain(t.bytes()));
        let mut result = 0;
        for c in s.bytes().chain(t.bytes()) {
            // 서로 다른 문자열이 단 한개차이 일 때 XOR연산을 사용하면 효과적임
            // x ^ y ^ y = x 속성을 이용해 초기값을 0으로 설정하고 XOR연산을 실행한다
            // 서로 같은 문자열을 XOR연산하게 되면 0이기 때문에 다른 하나의 문자는 0이 아닌 값을 가지게 됨
            result ^= c;
        }
        result as char
        

        //  s.as_bytes()
        //     .iter()
        //     .chain(t.as_bytes())
        //     .fold(0, |acc, &x| acc ^ x) as char
        
    }
}