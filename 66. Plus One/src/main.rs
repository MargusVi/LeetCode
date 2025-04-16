pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    println!("Input: {:?}", digits);

    let mut new_digits: Vec<i32> = digits;

    let mut index = new_digits.len();
    while index > 0 {
        index -= 1;
        if let Some(current) = new_digits.get_mut(index) {
            if *current == 9 {
                *current = 0;
                if index == 0 || new_digits.get_mut(index - 1).is_none() {
                    new_digits.insert(0, 1);
                }
                // println!("Current array: {:?}", new_digits);
            } else {
                *current += 1;
                break;
            }
        }
    }

    println!("Output: {:?}\n", new_digits);
    new_digits
}

fn main() {
    println!("Adding 1 to numbers represented as array:\n");

    let test_cases = vec![
        (vec![1, 2, 3]),
        (vec![4, 3, 2, 1]),
        (vec![9]),
        (vec![9, 9]),
        (vec![2, 9, 8, 9, 9]),
    ];

    for test_case in test_cases {
        plus_one(test_case);
    }
}
