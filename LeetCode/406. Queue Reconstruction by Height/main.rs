impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        let mut res = Vec::<Vec<i32>>::with_capacity(people.len());
        for p in people {
            res.insert(p[1] as usize, p);
        }
        res
    }
}