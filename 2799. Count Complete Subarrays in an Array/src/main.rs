use std::collections::HashSet;

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    println!("Input: {:?}", nums);
    let mut count = 0;
    let mut unique_array = Vec::new();
    for num in nums.clone() {
        if !unique_array.contains(&num) {
            unique_array.push(num);
        }
    }
    println!("Number of unique elements: {}", unique_array.len());
    for i in 0..nums.len() {
        let mut unique_elements = HashSet::new();
        for j in i..nums.len() {
            unique_elements.insert(nums[j]);
            if unique_elements.len() == unique_array.len() {
                count += 1;
                println!("{:?} (indexes {}..{})", &nums[i..=j], i, j);
            }
        }
    }

    println!("Output: {}\n", count);
    count
}

fn main() {
    println!("Counting complete subarrays:\n");

    let test_cases = vec![vec![1, 3, 1, 2, 2], vec![5, 5, 5, 5]];

    for test_case in test_cases {
        count_complete_subarrays(test_case);
    }
}
