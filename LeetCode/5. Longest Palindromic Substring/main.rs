use std::io;

fn main() {
    let s = "babad";
    let ans = Solution::longest_str_chain(words);
}


impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Manacher's algorithm
        let mut bytes = Vec::new();
        for b in s.bytes() {
            bytes.push(b'|');
            bytes.push(b);
        }
        bytes.push(b'|');
        let len = bytes.len();
        let mut radii = vec![0; len];
        let mut center = 0;
        let mut radius = 0;

        while center < len {
            while center >= radius + 1 && center + (radius + 1) < len && bytes[center - (radius + 1)] == bytes[center + (radius + 1)] {
                radius += 1;
            }

            radii[center] = radius;

            let mut old_center = center;
            let mut old_radius = radius;
            center += 1;
            radius = 0;
            while center <= old_center + old_radius {
                let mut mirror_center = old_center - (center - old_center);
                let mut mirror_radius = old_center + old_radius - center;
                if radii[mirror_center] < mirror_radius {
                    radii[center] = radii[mirror_center];
                    center += 1;
                } 
                else if radii[mirror_center] > mirror_radius {
                    radii[center] = mirror_radius;
                    center += 1;
                }
                else {
                    radius = mirror_radius;
                    break;
                }
            }
        }

        //return string
        let mut max_radius = 0;
        let mut max_center = 0;
        for (i, r) in radii.iter().enumerate() {
            if *r > max_radius {
                max_radius = *r;
                max_center = i;
            }
        }

        let mut ans = String::new();
        for i in (max_center - max_radius + 1..max_center + max_radius).step_by(2) {
            ans.push(bytes[i] as char);
        }
        ans
    }
}