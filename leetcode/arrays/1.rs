use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_indices = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&other_index) = num_indices.get(&complement) {
                return vec![other_index as i32, index as i32];
            }

            num_indices.insert(num, index);
        }

        vec![]
    }
}

