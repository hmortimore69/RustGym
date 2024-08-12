struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut start = 0;
        let mut max_len = 1;
        for i in 0..n {
            dp[i][i] = true;
            if i < n - 1 && s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                start = i;
                max_len = 2;
            }
        }
        for len in 3..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if s[i] == s[j] && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    start = i;
                    max_len = len;
                }
            }
        }
        String::from_utf8(s[start..start + max_len].to_vec()).unwrap()
    }
}

#[test]
fn test() {
    let s = "babad".to_string();
    let result = Solution::longest_palindrome(s);
    assert!(result == "bab".to_string() || result == "aba".to_string());

    let s = "cbbd".to_string();
    assert_eq!(Solution::longest_palindrome(s), "bb".to_string());
}