Problem located at https://leetcode.com/problems/paint-house-iii

## Notes
- Implementing this in Rust was far too frustrating. Switched to python just to make it easier on myself.

# Solution
This is a pretty complex 3D DP. The simple logic is that you are able to bash all possibilities of color choice as there is only 20 colors to choose from.
As a result, you can iterate choosing every color for every possible house that can be painted, then just keeping track of the neighborhood count along the way.

The total states in the dp are `(m * n * target)`, while each state has `n` different states (colors), giving a total of `O(n^2 * m * target)`.

Actually a pretty complex problem at first, but dp is op.