impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let mut stack: Vec<(i32, i32)> = Vec::new(); // (value, count)
        let mut result = 0;
        let mut current_sum = 0;

        for &value in arr.iter() {
            let mut count = 1;
            while let Some(&(prev_value, prev_count)) = stack.last() {
                if prev_value > value {
                    stack.pop();
                    count += prev_count;
                    current_sum -= prev_value * prev_count;
                } else {
                    break;
                }
            }
            stack.push((value, count));
            current_sum += value * count;
            current_sum %= modulo;
            result += current_sum;
            result %= modulo;
        }

        result
    }
}