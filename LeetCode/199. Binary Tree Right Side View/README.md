Problem located at https://leetcode.com/problems/binary-tree-right-side-view

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Simple BFS problem. You have to go through all the nodes anyway, because there is no guarantee which node will be the right-most for each level.

BFS solves this issue by just parsing each level at a time, using an inner while loop / length variable. Other than that, this problem is basically all 
implementation.