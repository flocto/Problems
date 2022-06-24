use std::collections::BinaryHeap;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        // work backwards from target, using max heap to prune and replace largest element
        let mut pq = BinaryHeap::new();
        let mut sum = 0;
        for &x in &target {
            // must be usize due to overflow
            // if only i32, then will overflow, causing negative values of sum/A, which creates infinite loop
            sum += x as usize;
            pq.push(x as usize);
        }
        while(!pq.is_empty()) { // loop {} can also be used, not much difference in performance from what I can tell
            let a = pq.pop().unwrap();
            sum -= a;
            
            // base array starts with all 1s, if A or sum is 1 then base case reached (sum means [1, A], A means [1,...,1])
            if a == 1 || sum == 1 { 
                return true;
            }

            // if A < sum, then A could not have been put into the array as A = sum + x where x is old value at position where A is
            // if sum == 0, then array must have all 0s, which is not possible
            if a < sum || sum == 0 { 
                return false;
            }

            // optimization: use A % sum instead of A - sum
            // this skips iterations where the position that A is at
            // is constantly replaced by the sum 
            // Ex: [1,1,1,1] -> [1,1,1,10], after above code, A will be 10, sum will be 3.
            // Using A - sum is essentially [1,1,1,10] -> [1,1,1,7] -> [1,1,1,4] -> [1,1,1,1]
            // Doing % skips this step as A essentially (sum * k) + x 
            // However, when doing this optimization, you have to be careful for cases where A % sum == 0
            // Ex: [1,1,1,9] -> [1,1,1,0]. 0 is not possible so such a case is automatically false
            let rem = a % sum;
            if rem == 0 {
                return false;
            }

            // add rem back to sum because its now an element of the array
            sum += rem;
            pq.push(rem);
        }
        false
    }
}