struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let (mut i_min, mut i_max, half_len) = (0, m, (m + n + 1) / 2);
        while i_min <= i_max {
            let i = (i_min + i_max) / 2;
            let j = half_len - i;
            if i < i_max && nums2[j - 1] > nums1[i] {
                i_min = i + 1;
            } else if i > i_min && nums1[i - 1] > nums2[j] {
                i_max = i - 1;
            } else {
                let max_of_left;
                if i == 0 {
                    max_of_left = nums2[j - 1];
                } else if j == 0 {
                    max_of_left = nums1[i - 1];
                } else {
                    max_of_left = nums1[i - 1].max(nums2[j - 1]);
                }
                if (m + n) % 2 == 1 {
                    return max_of_left as f64;
                }
                let min_of_right;
                if i == m {
                    min_of_right = nums2[j];
                } else if j == n {
                    min_of_right = nums1[i];
                } else {
                    min_of_right = nums1[i].min(nums2[j]);
                }
                return (max_of_left + min_of_right) as f64 / 2.0;
            }
        }
        
        0.0
    }
}


#[test]
fn test() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
}