Problem located at https://leetcode.com/problems/min-cost-climbing-stairs

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Really easy DP problem, just grab the minimum value of the previous two steps and use that to store the new dp value 
for the current step. 

Only special part is that there are 'invisible' steps with cost 0 before and after the array, you can manually add them
to the array or just do what I did and initialize 2 dp values as well as check last two values.