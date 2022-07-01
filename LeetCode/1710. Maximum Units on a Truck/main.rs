impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut stored = truck_size;
        let mut total = 0;
        for i in 0..box_types.len() {
            total += box_types[i][0] * box_types[i][1];
            stored -= box_types[i][0];
            if stored <= 0 {
                total += stored * box_types[i][1];
                break;
            }
        }
        total
    }
}