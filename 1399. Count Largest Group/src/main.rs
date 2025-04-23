use std::collections::HashMap;

pub fn count_largest_group(n: i32) -> i32 {
    println!("Input: n = {}", n);
    let mut num_count = HashMap::new();

    for i in 1..=n {
        let mut current = i;
        let mut digit_sum = 0;

        while current > 0 {
            digit_sum += current % 10;
            current /= 10;
        }

        *num_count.entry(digit_sum).or_insert(0) += 1;
    }

    let largest_digit_sum = *num_count.values().max().unwrap();
    println!(
        "Output: {}\n",
        num_count
            .values()
            .filter(|&&x| x == largest_digit_sum)
            .count() as i32
    );
    num_count
        .values()
        .filter(|&&x| x == largest_digit_sum)
        .count() as i32
}

fn main() {
    println!("Counting largest group:\n");

    let test_cases = vec![13, 2];

    for test_case in test_cases {
        count_largest_group(test_case);
    }
}
