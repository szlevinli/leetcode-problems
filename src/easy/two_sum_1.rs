pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = std::collections::HashMap::new();
        let mut ans = Vec::new();

        for (i, v) in nums.iter().enumerate() {
            let diff = target - v;

            match hash_map.get(&diff) {
                None => {
                    hash_map.insert(v, i);
                }
                Some(&value) => {
                    ans.push(value as i32);
                    ans.push(i as i32);
                    break;
                }
            }
        }

        ans
    }
}
