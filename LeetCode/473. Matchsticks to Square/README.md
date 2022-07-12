Problem located at https://leetcode.com/problems/matchsticks-to-square

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
As there are only 15 matches, it is reasonable to do a `O(4 ^ n)` solution as there is no way to solve this problem
in polynomial time. This is because the problem essentially boils down to splitting the array into 4 subsequences with equal sum.
A free optimization is just checking if the sum if divisible by 4 as it has to be to form a square.

The rest of the solution is just brute forcing all possibilities of which side a match can be on, and just returning true if
any possible combination works.

Optimized code is taken directly from [here.](https://leetcode.com/problems/matchsticks-to-square/discuss/1275432/Rust%3A-Back-tracking)