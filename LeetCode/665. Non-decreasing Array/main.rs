impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut count = 0;
        for i in 1..nums.len() {
            if count > 1 {
                return false;
            }
            if nums[i - 1] > nums[i] {
                count += 1;
                // main part of algorithm
                // if i-1 is greater than i, then either i-1 or i must be changed
                // i == 1 just covers case where nums[0] > nums[1], which means nums[1] has to be changed to nums[0] to be non-decreasing
                // all other cases, nums[0...i-1] must be non-decreasing
                // this means nums[i - 2] must be less than or equal to nums[i - 1]
                //
                // if nums[i - 2] is less than or equal to nums[i], then nums[i - 1], which is greater than both nums[i - 2] and nums[i],
                // must be changed to nums[i] (or any value between nums[i - 2] and nums[i])
                //
                // otherwise, if nums[i - 2] is greater than nums[i], then nums[i] has to be changed,
                // as nums[i - 2] <= nums[i - 1] <= nums[i] must be true. nums[i] can be set to any value between nums[i - 1] and nums[i + 1],
                // but of course nums[i + 1] is not a guaranteed value. 
                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
            }
        }
        count <= 1
    }
}