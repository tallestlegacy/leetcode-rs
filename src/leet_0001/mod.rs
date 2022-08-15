pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut i = 0;

    while i < nums.len() - 1 {
        let mut j = i.clone() + 1;

        while j < nums.len() {
            if nums[i] + nums[j] == target {
                ans.push(i as i32);
                ans.push(j as i32);
                return ans;
            }
            j += 1;
        }
        i += 1;
    }

    return ans;
}
