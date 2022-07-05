Problem located at https://leetcode.com/problems/longest-consecutive-sequence

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Very simple solution using a set. A smart optimization I saw in the discussion was checking if `n - 1` existed in the set. 
If it does exist, that means you can ignore the current number as a larger sequence exists that uses `n - 1`.