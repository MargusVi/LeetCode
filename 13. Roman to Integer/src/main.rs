pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;

    let value_map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    // println!("\n");

    let mut result: i32 = 0;

    for (index, char) in s.char_indices() {
        let value = *value_map.get(&char).unwrap_or(&0);

        if (char == 'I'
            && (s.chars().nth(index + 1) == Some('V') || s.chars().nth(index + 1) == Some('X')))
            || (char == 'X'
                && (s.chars().nth(index + 1) == Some('L') || s.chars().nth(index + 1) == Some('C')))
            || (char == 'C'
                && (s.chars().nth(index + 1) == Some('D') || s.chars().nth(index + 1) == Some('M')))
        {
            result -= value;
        } else {
            result += value;
        }

        // println!(
        //     "char: {:?}\tvalue: {}\tnext char: {:?}\tcurrent result: {}",
        //     char,
        //     value,
        //     s.chars().nth(index + 1),
        //     result
        // );
    }

    result
}

fn main() {
    println!("Converting roman numerals to integers:\n");
    let test_cases = vec![
        "III".to_string(),
        "LVIII".to_string(),
        "MCMXCIV".to_string(),
    ];

    for case in test_cases {
        println!("Input: {}\tOutput: {}", case.clone(), roman_to_int(case));
    }
}
