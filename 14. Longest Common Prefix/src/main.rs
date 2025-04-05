pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut index = 0;

    loop {
        let current_char = match strs[0].chars().nth(index) {
            None => break,
            Some(c) => c,
        };

        for str in &strs[1..] {
            match str.chars().nth(index) {
                None => return result,
                Some(c) if c != current_char => return result,
                _ => {}
            }
        }

        result.push(current_char);
        index += 1;
    }

    result
}

fn main() {
    println!("Longest common prefix string amongst an array of strings:\n");

    let input = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!(
        "Input: {:?}\nOutput: {}\n",
        input.clone(),
        longest_common_prefix(input),
    );

    let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    println!(
        "Input: {:?}\nOutput: {}\n",
        input.clone(),
        longest_common_prefix(input),
    );
}
