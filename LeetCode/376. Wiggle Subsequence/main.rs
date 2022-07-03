impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut greater = vec![0; n];
        let mut less = vec![0; n];
        greater[0] = 1;
        less[0] = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                greater[i] = less[i - 1] + 1; // use value from previous less
                less[i] = less[i - 1];
            } else if nums[i] < nums[i - 1] {
                greater[i] = greater[i - 1];
                less[i] = greater[i - 1] + 1;
            } else {
                greater[i] = greater[i - 1];
                less[i] = less[i - 1];
            }
        }
        std::cmp::max(greater[n - 1], less[n - 1]) as i32
    }
}