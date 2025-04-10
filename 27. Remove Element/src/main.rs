pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut pop_indexes: Vec<i32> = Vec::new();

    println!("Input: nums = {:?}, val = {}", nums, val);
    for (index, &num) in nums.iter().enumerate() {
        if num == val {
            pop_indexes.push(index.try_into().unwrap());
        }
    }

    pop_indexes.sort_by(|a, b| b.cmp(a));
    for &index in &pop_indexes {
        nums.remove(index as usize);
    }

    println!("Output: {}, nums = {:?}\n", nums.len(), nums);
    nums.len() as i32
}

// // O método abaixo também funciona:
// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     nums.retain(|&x| x != val);
//     nums.len() as i32
// }

fn main() {
    println!("Removing all of one specific element from integer arrays:\n");

    let test_cases = vec![(vec![3, 2, 2, 3], 3), (vec![0, 1, 2, 2, 3, 0, 4, 2], 2)];

    for (mut nums, val) in test_cases {
        remove_element(&mut nums, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        // Test cases: (input array, value to remove, expected array after removal)
        let test_cases = vec![
            (vec![3, 2, 2, 3], 3, vec![2, 2]),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4]),
        ];

        for (mut nums, val, expected) in test_cases {
            let k = remove_element(&mut nums, val);

            // Assert k equals the length of expected array
            assert_eq!(k as usize, expected.len());

            // Sort both arrays for comparison
            nums.sort();
            let mut expected_sorted = expected.clone();
            expected_sorted.sort();

            // Compare each element
            for i in 0..k as usize {
                assert_eq!(nums[i], expected_sorted[i]);
            }
        }
    }
}
