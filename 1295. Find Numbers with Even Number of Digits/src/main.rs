pub fn find_numbers(nums: Vec<i32>) -> i32 {
    println!("Input: nums = {:?}", nums);
    let mut count = 0;

    for num in nums {
        let num_digits = num.to_string().len();
        print!("{} -> {} digits -> ", num, num_digits);
        if num_digits % 2 == 0 {
            count += 1;
            println!("even");
        } else {
            println!("odd");
        }
    }

    println!("Output: {}\n", count);
    count
}

fn main() {
    println!("Finding numbers with even number of digits:\n");

    let test_cases = vec![vec![12, 345, 2, 6, 7896], vec![555, 901, 482, 1771]];

    for test_case in test_cases {
        find_numbers(test_case);
    }
}
