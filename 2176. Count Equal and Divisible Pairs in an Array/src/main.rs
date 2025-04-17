pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    println!("Input: {:?}", nums);
    let mut count = 0;
    for (index1, num1) in nums.clone().iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if index1 < index2 {
                // println!("{} ({}) - {} ({})", num1, index1, num2, index2);
                if num1 == num2 && ((index1 * index2) % k as usize) == 0 {
                    count += 1;
                }
            }
        }
    }
    println!("Output: {}\n", count);
    count
}

fn main() {
    println!("Counting pairs:\n");

    let test_cases = vec![(vec![3, 1, 2, 2, 2, 1, 3], 2), (vec![1, 2, 3, 4], 1)];

    for (nums, k) in test_cases {
        count_pairs(nums, k);
    }
}
