// 1. Two Sum
// https://leetcode.com/problems/two-sum/description/

fn main() {
    let num = vec![2, 7, 11, 15];

    println!("Two sum >> {:?}", two_sum(num, 9))
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let len : usize= nums.len();
    //  outer iterator
    let mut i: usize = 0;
    while i < len - 1 {
        //  inner iterator
        let mut j: usize = i + 1;
        while j < len {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
            j += 1;
        }
        i += 1;
    }
    vec![0, 0]
}
