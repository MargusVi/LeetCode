pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    println!("Input: nums = {:?}, target = {}", nums, target);
    if let Some(index) = nums.iter().position(|&num| num == target) {
        println!("Output: {}", index as i32);
        index as i32
    } else if let Some(index) = nums.iter().position(|&num| num > target) {
        println!("Output: {}", index as i32);
        index as i32
    } else {
        println!("Output: {}", nums.len() as i32);
        nums.len() as i32
    }
}

fn main() {
    println!("Searching import position:\n");

    let test_cases = vec![
        (vec![1, 3, 5, 6], 5),
        (vec![1, 3, 5, 6], 2),
        (vec![1, 3, 5, 6], 7),
        (vec![1, 3, 5, 6], 0),
    ];

    for (nums, target) in test_cases {
        search_insert(nums, target);
    }
}
