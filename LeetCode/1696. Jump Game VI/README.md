Problem located at https://leetcode.com/problems/jump-game-vi

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
[This discussion](https://leetcode.com/problems/jump-game-vi/discuss/1260737/Optimizations-from-Brute-Force-to-Dynamic-Programming-w-Explanation) was too good not to implement.
My original solution was the 4th one show in that discussion, but the 5th one was just way too good.

The solution is essentially a DP used to store results and a Deque to keep track of possible indices that can be visited. It is similar to pruning a BFS.

The top element of the deque stores the index that was considered last. If that index is unreachable, then it can be popped from the deque.
Then, all previous elements that have a DP value less than the current index are also pruned, as they can always be visited from the previous index (the top of the deque) that
visits the current index, meaning the current index is the best choice out of all `k` steps from the previous index. Finally, the index is just added to the deque.

At the end of the for loop, all possible positions would have been visited only once, meaning it is an `O(n)` solution both time and space-wise.
Very very cool and clever solution, please check out that link.