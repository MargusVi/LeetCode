pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    println!("Input: arr = {:?}, a = {}, b = {}, c = {}", arr, a, b, c);
    let mut good_triplets = 0;

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            for k in j + 1..arr.len() {
                if (arr[i] - arr[j]).abs() <= a
                    && (arr[j] - arr[k]).abs() <= b
                    && (arr[i] - arr[k]).abs() <= c
                {
                    good_triplets += 1;
                }
            }
        }
    }

    println!("Output: {}", good_triplets);
    good_triplets
}

fn main() {
    println!("Counting good triplets:\n");

    let test_cases = vec![
        (vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
        (vec![1, 1, 2, 2, 3], 0, 0, 1),
    ];

    for (arr, a, b, c) in test_cases {
        count_good_triplets(arr, a, b, c);
    }
}
