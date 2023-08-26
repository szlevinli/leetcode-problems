pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        fn to_string(low: i32, high: i32) -> String {
            if low == high {
                format!("{low}")
            } else {
                format!("{low}->{high}")
            }
        }

        let len = nums.len();
        let mut low = 0;
        let mut ans = Vec::new();

        while low < len {
            let mut high = low;

            while high + 1 < len && nums[high] == nums[high + 1] - 1 {
                high += 1;
            }

            ans.push(to_string(nums[low], nums[high]));
            low = high + 1;
        }

        ans
    }
}
