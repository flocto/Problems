impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        // count character frequency
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }
        //max sort frequency
        freq.sort_by(|a, b| b.cmp(a)); 
        let mut ans = 0;
        let mut min = freq[0];
        for i in 0..26 {
            if freq[i] == 0 { break; } // freq[i] == 0 means no more chars
            if freq[i] > min { // reduce down to min
                ans += freq[i] - min;
                freq[i] = min;
            }
            min = freq[i]; // update min
            if min > 0 { min -= 1; } // minimum min is 0
        }
        ans
    }
}