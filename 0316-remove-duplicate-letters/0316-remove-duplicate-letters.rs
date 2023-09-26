impl Solution {
  pub fn remove_duplicate_letters(s: String) -> String {
    let s = s.into_bytes();
    let mut last_pos = [0; 26];
    
    // 각 문자열의 마지막 위치 저장
    for i in 0 .. s.len() {
      last_pos[s[i] as usize - 97] = i as i32;
    }

    // 알파벳 길이만큼의 스택 생성
    let mut stack = Vec::with_capacity(26);

    for i in 0 .. s.len() {
      let c = s[i] as usize - 97;
      if stack.contains(&c) {
        continue;
      }

      // 아래 조건을 만족한다는 것은 사전순으로 더 작은 문자열이 생성된다는 것.
      // 스택의 마지막 요소가 현재 문자열보다 크다는 것은 사전순으로 크다는 것.
      // 마지막 조건은 문자가 입력 문자열에서 나중에도 찾을 수 있으므로 스택에서 제거해도 안전함을 보장
      while !stack.is_empty() && stack[stack.len() - 1] > c && last_pos[stack[stack.len() - 1]] as usize > i {
        stack.pop();
      }
      stack.push(c);
    }

    String::from_utf8(stack.iter().map(|x| *x as u8 + 97).collect::<Vec<u8>>()).unwrap()
  }
}