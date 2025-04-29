pub fn check_if_exist(arr: Vec<i32>) -> bool {
    println!("Input: arr = {:?}", arr);

    let n = arr.len();
    for i in 0..n {
        for j in (i + 1)..n {
            // Indices i and j are guaranteed to be different (j > i)
            let num_i = arr[i];
            let num_j = arr[j];
            println!("Indices ({}, {}) -> Numbers ({}, {})", i, j, num_i, num_j);

            // Check if one number is the double of the other
            if num_i == 2 * num_j || num_j == 2 * num_i {
                println!(
                    "Found {} and {} -> one is the double of the other",
                    num_i, num_j
                );
                println!("Output: true\n");
                return true;
            }
        }
    }

    println!("Output: false\n");
    false
}

fn main() {
    println!("Checking if N and its double exist:\n");

    let test_cases = vec![vec![10, 2, 5, 3], vec![3, 1, 7, 11]];

    for test_case in test_cases {
        check_if_exist(test_case);
    }
}
