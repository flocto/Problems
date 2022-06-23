use std::collections::BinaryHeap;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by(|a, b| a[1].cmp(&b[1])); // sort by deadline
        let mut heap = BinaryHeap::new();
        let mut time = 0;
        for course in courses {
            time += course[0];
            heap.push(course[0]);
            if time > course[1] { // if time exceeds deadline, remove the longest course
                time -= heap.pop().unwrap();
            }
        }
        heap.len() as i32
    }
}