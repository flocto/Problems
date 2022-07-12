impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        sum % 4 == 0 && Self::solve(&mut matchsticks[..], 0, 0, 1, sum / 4)
    }

    fn solve(sticks: &mut [i32], l: usize, len: i32, side: i32, max: i32) -> bool {
        side == 4 // sides all filled
            || len == max && Self::solve(sticks, 0, 0, side + 1, max) // this side filled
            || (l..sticks.len()).any(|i| { // try next stick (i is index)
                let stick = std::mem::take(&mut sticks[i]); // rust ownership
                let res = stick != 0 && Self::solve(sticks, i + 1, len + stick, side, max); // if the stick isnt null and it works
                sticks[i] = stick; // put stick back (rust ownership)
                res
            })
    }
}