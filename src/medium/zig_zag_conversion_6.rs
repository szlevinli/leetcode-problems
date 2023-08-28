/// Zigzag Conversion
/// 中文称作 Z字型变换
/// 就是将给定的字符按如下形式排列.
/// 下面这个按3行
///
/// |   |   |   |
/// | / | / | / |
/// |   |   |
///
/// 下面按4行
///
/// |     |     |
/// |   / |   /
/// | /   | /
/// |     |
pub struct Solution;

impl Solution {
    /// 这里使用高赞版本的解题思路
    /// 使用一个step标志位来表示移动方向
    /// 从上往下step=1, 从下往上step=-1
    ///
    /// 使用一个一维数组, 数组中的值是一个字符串
    /// 数组中的第一个代表第一行, 第二个代表第二行, 依次类推
    ///
    /// 需要空置的就是step的方向, +1或-1
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 || num_rows > s.len() as i32 {
            return s;
        }

        let mut ans = vec![String::new(); num_rows as usize];
        let mut i = 0;
        let mut step = -1;

        s.chars().for_each(|v| {
            ans[i as usize].push(v);
            if i == 0 || i == num_rows - 1 {
                step = -step;
            }
            i += step;
        });

        ans.join("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_input_string_is_empty() {
        let input = String::new();
        let row_nums = 3;
        let expected_output = String::new();

        assert_eq!(Solution::convert(input, row_nums), expected_output);
    }

    #[test]
    fn it_row_nums_great_than_string_length() {
        let input = String::from("abc");
        let row_nums = 4;
        let expected_output = String::from("abc");

        assert_eq!(Solution::convert(input, row_nums), expected_output);
    }

    #[test]
    fn it_example_1() {
        let input = String::from("PAYPALISHIRING");
        let row_nums = 3;
        let expected_output = String::from("PAHNAPLSIIGYIR");

        assert_eq!(Solution::convert(input, row_nums), expected_output);
    }

    #[test]
    fn it_example_2() {
        let input = String::from("PAYPALISHIRING");
        let row_nums = 4;
        let expected_output = String::from("PINALSIGYAHRPI");

        assert_eq!(Solution::convert(input, row_nums), expected_output);
    }
}
