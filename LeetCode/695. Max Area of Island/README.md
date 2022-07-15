Problem located at https://leetcode.com/problems/max-area-of-island

## Notes
- Java time!

# Solution
Surprise. I did this one in Java. 
LOL ok jokes aside, this one is actually pretty simple.
It's mostly just floodfill combined with a counter.

DFS or BFS both work, just as long as you keep track of visited nodes to prevent recounting already counted islands. After that, its just a simple loop
over the entire map, flood-filling when you reach a 1.