Problem located at https://leetcode.com/problems/max-area-of-island

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
This problem is a headache in Rust... too much ownership stuff.
Finally got it working by basically frankensteining community solutions.

Anyway, my solution uses a stack to keep track of the nodes. It operates a lot like DFS, but uses the pre-order
to keep track of what node needs to be considered next. If you want a better explanation, reading some of the community posts is 
probably a better bet.