Problem located at https://leetcode.com/problems/maximum-units-on-a-truck/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Very simple problem and solution, just sorting by value and taking boxes greedily. You can optimize a little by 
adding an entire category of boxes at once instead of one at a time, then checking if limit is exceeded after.