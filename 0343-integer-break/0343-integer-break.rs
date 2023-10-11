impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // 1 = 1 + 0,
        // 2 = 1 * 1 -> 1
        // 3 = 1 * 2 -> 2
        // 4 = 2 * 2 -> 4
        // 5 = 3 * 2 -> 6
        // 6 = 3 * 3 -> 9
        // 7 = 3 * 2 * 2 -> 12
        // 8 = 3 * 3 * 2 -> 18
        // 9 = 3 * 3 * 3 -> 27
        // 10 = 3 * 3 * 2 * 2 -> 36
        // 11 = 3 * 3 * 3 * 2 -> 45
        // 12 = 3 * 3 * 3 * 3 -> 81
        // 16 = 3 * 3 * 3 * 3 * 2 * 2
        
        if n < 4 {
            return n - 1;
        }
        
        // 3이 반복되는 횟수
        let count_of_3s = n / 3;
        
        // 나머지
        let remainder = n % 3;

        if remainder == 0 {
            return 3i32.pow(count_of_3s as u32);
        }

        if remainder == 1 {
            return 3i32.pow(count_of_3s as u32 - 1) * 4;
        }

        return 3i32.pow(count_of_3s as u32) * 2;
    }
}