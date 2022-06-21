use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        let ladders = ladders as usize;
        let mut used_brick = 0;
        for i in 0..heights.len() - 1 {
            let i = i as usize;
            let diff = heights[i+1] - heights[i];
            if diff > 0 {
                pq.push(Reverse(diff)); // rust binary heap is max heap, use cmp::Reverse to turn into min heap
            } 
            if pq.len() > ladders {
                used_brick += pq.pop().unwrap().0;
            }
            if used_brick > bricks {
                return i as i32;
            }
        }
        return (heights.len() - 1) as i32;
    }
}