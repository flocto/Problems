Problem located at https://leetcode.com/problems/candy/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Simple two passes solution, first pass makes sure higher right child recieves more, second pass makes sure higher left child recieves more.
Second pass uses max to ensure the first pass is not violated.