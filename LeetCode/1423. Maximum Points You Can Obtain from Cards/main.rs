impl Solution {
    /*There are several cards arranged in a row, and each card has an associated number of points. The points are given in the integer array cardPoints.

    In one step, you can take one card from the beginning or from the end of the row. You have to take exactly k cards.

    Your score is the sum of the points of the cards you have taken.

    Given the integer array cardPoints and the integer k, return the maximum score you can obtain.
    */

    // O(NK) solution
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        //generate both suffix and prefix sums up to subarray of length k
        let mut prefix = vec![0; card_points.len() + 1];
        let mut suffix = vec![0; card_points.len() + 1];
        for i in 1..k + 1  {
            prefix[i] = prefix[i - 1] + card_points[i - 1];
        }
        for i in (card_points.len() - k..card_points.len()).rev()  {
            suffix[i] = suffix[i + 1] + card_points[i];
        }

        //just manually check every pair of prefix and suffix sums that can be obtained from k cards
        let mut max = 0;
        for i in 0..k + 1  {
            max = std::cmp::max(max, prefix[i] + suffix[card_points.len() - k + i]);
        }
        max
    }
}