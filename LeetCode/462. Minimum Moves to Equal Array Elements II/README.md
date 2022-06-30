Problem located at https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Solution is essentially just [this.](https://math.stackexchange.com/questions/113270/the-median-minimizes-the-sum-of-absolute-deviations-the-ell-1-norm)

After sorting the entire array, the median will be in the middle (obviously). Then, we only have to iterate
from 0 to len/2, and then return the sum of the differences between nums\[len - i - 1] and nums\[i].

The difference between these two values is equal to the number of steps to make both equal to the median.
This is a very simple deduction, if you order the 3 numbers a, b, c, then `(b - a) + (c - b) = c - a`.
As the array is sorted, you know nums\[len - i - 1] has to be greater than or equal to nums\[i], so the statement must be true.


After looking at other solutions, there also appears to be an answer finding the median with the algorithm implemented in [215.](https://github.com/flocto/Problems/tree/main/LeetCode/215.%20Kth%20Largest%20Element%20in%20an%20Array)
However, I think the first solution is good enough to pass interviews.