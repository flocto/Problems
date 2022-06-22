use rand::thread_rng;
use rand::seq::SliceRandom;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.shuffle(&mut rand::thread_rng());
        //based on this solution 
        //https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/60294/Solution-explained
        let k = nums.len() - k as usize;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = Solution::partition(&mut nums, left, right);
            if mid == k {
                break;
            } else if mid < k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        nums[k]
    }
    pub fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let pivot = nums[right];
        let mut i = left;
        for j in left..right {
            if nums[j] < pivot {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i, right);
        i
    }
}