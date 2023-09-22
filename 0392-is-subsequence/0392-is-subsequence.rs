impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
          match iter.find(|&p| p == c) {//find 메서드는 이미 찾은 위치 이후부터 검색을 하므로 자동으로 순차검색이 됨
            Some(_) => (),
            None => return false
          }
        }
        true
    }
}