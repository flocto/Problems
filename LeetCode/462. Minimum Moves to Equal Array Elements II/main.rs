impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = 0;
        for i in 0..n / 2 { 
            res += nums[n - i - 1] - nums[i];
        }
        res
    }
}