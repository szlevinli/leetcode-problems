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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_empty() {
        let input = Vec::new();
        let expected_output: Vec<String> = Vec::new();

        assert_eq!(Solution::summary_ranges(input), expected_output);
    }

    #[test]
    fn it_example_1() {
        let input = vec![0, 1, 2, 4, 5, 7];
        let expected_output = vec!["0->2", "4->5", "7"];

        assert_eq!(Solution::summary_ranges(input), expected_output);
    }

    #[test]
    fn it_example_2() {
        let input = vec![0, 2, 3, 4, 6, 8, 9];
        let expected_output = vec!["0", "2->4", "6", "8->9"];

        assert_eq!(Solution::summary_ranges(input), expected_output);
    }
}
