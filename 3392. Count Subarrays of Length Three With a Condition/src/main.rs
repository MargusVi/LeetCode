pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    println!("Input: {:?}", nums);
    let mut count = 0;

    for (index, num) in nums.iter().enumerate() {
        if nums.get(index + 2).is_none() {
            break;
        }
        let num2 = nums.get(index + 1).unwrap();
        let num3 = nums.get(index + 2).unwrap();
        // println!("{}, {}, {}", num, num2, num3);
        if 2 * (num + num3) == *num2 {
            // println!(
            //     "2 * ({} + {}) = {}, num2 = {}",
            //     num,
            //     num3,
            //     2 * (num + num3),
            //     num2
            // );
            count += 1;
        }
    }

    println!("Output: {}\n", count);
    count
}

fn main() {
    println!("Counting subarrays of Length three with a condition:\n");

    let test_cases = vec![vec![1, 2, 1, 4, 1], vec![1, 1, 1], vec![-1, -4, -1, 4]];

    for test_case in test_cases {
        count_complete_subarrays(test_case);
    }
}
