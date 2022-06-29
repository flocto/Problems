Problem located at https://leetcode.com/problems/queue-reconstruction-by-height/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
Sort by height first, then by the number of blockers.
This way, the tallest people are in the front, and the tallest
HAS to have 0 blockers because there is no one taller.

Then, every additional person can be insert at the the index
equal to the number of blockers, because as they are added to the result,
every value in the array is already greater than or equal to them.
They can just be inserted where there are k values in front of them.

Every value that is added afterwards will either be shorter, or the same height with more blockers.
If they are shorter, they aren't considered by any past element, and if they are the same height,
then they will just be inserted behind them because following elements have to have at least the same number of blockers.
It is impossible for two of the same height to have the same number of blockers as one will have to be behind another.