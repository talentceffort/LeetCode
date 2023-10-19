use std::collections::VecDeque;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::get_result(s) == Self::get_result(t)
    }

    pub fn get_result(s: String) -> VecDeque<char> {
            let mut stack: VecDeque<char> = VecDeque::new();
            for x in s.chars() {
                if x == '#' {
                    stack.pop_back();
                    continue;
                }
                stack.push_back(x);
            }
            stack
        }
}