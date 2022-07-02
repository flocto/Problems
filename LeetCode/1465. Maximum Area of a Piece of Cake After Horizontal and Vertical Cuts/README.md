Problem located at https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/

## Notes:
- Did not create an entire project as Rust can be run remotely from leetcode.

# Solution
At first, it seems like a simple sweep line problem. However, given constraits, sweep line would take O(n^2). 
This exceeds time limit, meaning a better solution is needed.

The main insight is that every section between parallel cuts intersects all sections of the perpendicular cuts.
This means the section with the maximum height will always intersect the section with the maximum width, creating
the largest piece overall.

The solution from here is very simple. Just insert 0 and w/h into their respective vectors, sort, then find the largest section.
Multiply the two sections together, take a mod, and that's the answer.
The solution could probably be optimized in Rust, but I'm still fairly new and there weren't any other Rust answers so idk.

The only issue on strongly-typed languages like C++, Java, and Rust that you have to worry about is integer overflow, just a simple cast
will fix it though.