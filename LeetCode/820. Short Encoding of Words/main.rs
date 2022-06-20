use std::collections::HashSet;
impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_by(|a, b| b.len().cmp(&a.len()));
        /* the .fold() function is used to iterate over the elements of the vector.
            it takes 2 arguments: a starting value, and a closure.
            the closure takes 2 arguments: the stored value, and the current element of the iterator.
            it stores the output of the included function in the stored value after acting on the current element.
            the stored value is returned at the end of the iteration, causing a single value to be returned from the entire vector.

            in this case, the closure just checks if any string in the set contains the current string as a suffix.
            if not, the length of the word, and an invisible '#' are added to the final length.
            in the end, the function returns the first value in the tuple, which is the final length of the encoding string.
        */
        words.iter().fold((0usize, HashSet::<&str>::new()), |(mut res, mut keys), word| {
            if keys.iter().all(|key| !key.ends_with(word)) {
                res += word.len()+1;
                keys.insert(word);
            }
            (res, keys)
        }).0 as i32

        // My original code for this solution, however learned how to use .fold() from other solutions.
        //
        // let mut set = HashSet::<&str>::new();
        // let mut res = 0;
        // for word in words.iter() {
        //     if set.iter().all(|key| !key.ends_with(word)) {
        //         res += word.len()+1;
        //         set.insert(word);
        //     }
        // }
        // res as i32
    }
}