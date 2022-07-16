Problem located at https://leetcode.com/problems/out-of-boundary-paths

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Use a DP to keep track of already visited tiles, and then whenever a move leaves the board, add it to the result.
Then just iterate over all possible moves over all possible tiles.

Sometimes the simplest answer is the correct one :). (Bounds are too low to suggest otherwise)