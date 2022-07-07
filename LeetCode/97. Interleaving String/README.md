Problem located at https://leetcode.com/problems/interleaving-string

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
A 2D DP problem. Because length of `s1` and `s2` are limited to 100, `O(n^2)` is okay in this case.
The 2 base cases for the DP are when both `s1` and `s2` are empty, and when only one of them is empty.
To deal with these cases, `dp[0][0]` is set to true, and 2 for loops are run that simulate either string being empty.
These loops also exit early to save time.

The last nested `O(n^2)` is the main DP. If either string matches `s3` at that specific letter at `i` or `j`, then if their
previous state was possible, the current state gets set to true. At the end, the state representing the end of both strings 
is returned. If true, then it is possible to interleave, otherwise it's not.