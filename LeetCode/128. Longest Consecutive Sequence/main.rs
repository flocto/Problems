impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let mut set: HashSet<i32> = HashSet::from_iter(nums);
        
        let max = set.iter().map(|x| {
            let mut cur = 0;
            let mut num = *x;
            if set.contains(&(x - 1)) {
                0
            }
            else {
                while set.contains(&(num)) {
                    cur += 1;
                    num += 1;
                }
                cur + 1
            }
        }).max().unwrap_or(0);
        
        max
    }
}