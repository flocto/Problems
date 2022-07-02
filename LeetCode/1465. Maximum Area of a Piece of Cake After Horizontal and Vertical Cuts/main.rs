impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut h_cuts: Vec<i64> = horizontal_cuts.iter().map(|&x| x as i64).collect();
        let mut v_cuts: Vec<i64> = vertical_cuts.iter().map(|&x| x as i64).collect();
        h_cuts.push(0);
        h_cuts.push(h as i64);
        v_cuts.push(0);
        v_cuts.push(w as i64);
        h_cuts.sort();
        v_cuts.sort();
        let max_height = get_max_dist(h_cuts);
        let max_width = get_max_dist(v_cuts);
        let mut max_area = max_height * max_width;
        (max_area % 1_000_000_007) as i32
    }
    pub fn get_max_dist(vec: Vec<i64>) -> i64 {
        let mut max = 0;
        for i in 0..vec.len()-1{
            max = max.max(vec[i+1]-vec[i]);
        }
        max
    }
}