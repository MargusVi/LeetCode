pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    println!("Input: {:?}", nums);
    let mut last_num: Option<i32> = None;
    let mut pop_indexes: Vec<i32> = Vec::new();

    for (index, num) in nums.iter().enumerate() {
        if last_num.is_some() && *num == last_num.unwrap() {
            pop_indexes.push(index.try_into().unwrap());
        }
        last_num = Some(*num);
    }

    pop_indexes.sort_by(|a, b| b.cmp(a));
    for &index in &pop_indexes {
        nums.remove(index as usize);
    }
    println!("Output: {}, nums = {:?}\n", nums.len(), nums);
    nums.len() as i32
}

fn main() {
    println!("Removing duplicates from sorted array:\n");

    let sorted_arrays = vec![vec![1, 1, 2], vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]];

    for mut sorted_array in sorted_arrays {
        remove_duplicates(&mut sorted_array);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let test_cases = vec![
            (vec![1, 1, 2], vec![1, 2]),
            (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]),
        ];

        for (mut nums, expected) in test_cases {
            let k = remove_duplicates(&mut nums);
            assert_eq!(k as usize, expected.len());
            for i in 0..k as usize {
                assert_eq!(nums[i], expected[i]);
            }
        }
    }
}
