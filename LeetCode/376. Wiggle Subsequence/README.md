Problem located at https://leetcode.com/problems/wiggle-subsequence/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
This is a simple DP problem using 2 arrays (can also be 1 2d one but this is Rust).
One DP keeps track of subsequences where `nums[i] < nums[i - 1]`, the other keeps track of `nums[i] > nums[i + 1]`.

The only issue is when the numbers are equal, but simply ignoring such a case (setting dp[i] to dp[i - 1]) solves that issue.
Otherwise, its just a simple iteration, updating each DP and taking values from the other DP to simulate the flipping of the comparison.