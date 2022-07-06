Problem located at https://leetcode.com/problems/fibonacci-number

## Notes
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Finally get to show off this formula lol. It's taken directly from [this Wikipedia page](https://en.wikipedia.org/wiki/Generalizations_of_Fibonacci_numbers#Higher_orders)
at the bottom. I found out about this formula from HSCTF 9, so shoutout to them :). But anyway, the first 3 solutions are probably
how most interviewees would solve them. The 4th solution is clever and takes some knowledge of LA. 

But the 5th solution :P...

It still might not be an O(1) solution because of how `powi()` might be defined. However, it probably is faster than many solutions for large values of n. 
Also Binet's probably works just as well (maybe even better because this is a generalization), but for both, as n increases, you have to have more bits
of precision to make sure you don't lose accuracy.