pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| b[0].cmp(&a[0]));

        while let Some(current) = intervals.pop() {
            if let Some(prev) = result.last_mut() {
                if current[0] <= prev[1] && prev[1] < current[1] {
                    prev[1] = current[1];
                } else if prev[1] < current[0] {
                    result.push(current.clone());
                }
            } else {
                result.push(current.clone());
            }
        }

        result
    }
}
