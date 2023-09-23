use std::collections::HashMap;
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        
        let mut words = words;
        words.sort_by_key(|word| word.len());

        let mut dp: HashMap<String, i32> = HashMap::new();
        let mut max_chain = 0;

        for word in &words {
            dp.insert(word.to_string(), 1);
            for i in 0..word.len() {
                // "ca", "ba", "bc"
                let prev_word = format!("{}{}", &word[..i], &word[i + 1..]);
                if let Some(val) = dp.get(&prev_word) {
                    // 누적값 저장해나가는 방식. 
                    // hashmap은 memo 역할
                    dp.insert(word.to_string(), std::cmp::max(dp[word], val + 1));
                }
            }
            max_chain = max_chain.max(dp[word]);
            
        }
        max_chain
    }
}