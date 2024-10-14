#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let s = s.as_bytes();
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];
        let mut i = 0;
        let mut flag = -1;

        for &c in s {
            rows[i].push(c as char);
            if i == 0 || i == num_rows - 1 {
                flag = -flag;
            }
            i = (i as i32 + flag) as usize;
        }

        rows.concat()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
}