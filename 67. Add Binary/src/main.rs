pub fn add_binary(a: String, b: String) -> String {
    println!("Input: a = \"{}\", b = \"{}\"", a, b);
    // println!();
    let larger_len = if a.len() > b.len() { a.len() } else { b.len() };
    let mut carry = '0';
    let mut result = String::new();
    let a_chars: Vec<char> = a.chars().rev().collect();
    let b_chars: Vec<char> = b.chars().rev().collect();
    for index in 0..larger_len {
        let mut current: Vec<char> = Vec::new();
        let a_char = if index < a_chars.len() {
            a_chars[index]
        } else {
            '0'
        };
        let b_char = if index < b_chars.len() {
            b_chars[index]
        } else {
            '0'
        };
        current.push(a_char);
        current.push(b_char);
        current.push(carry);
        let mut count_1 = 0;

        for element in current {
            // print!("{} ", element);
            if element == '1' {
                count_1 += 1;
            }
        }
        // println!("-> 1s = {}", count_1);
        if count_1 == 0 {
            result.push('0');
            carry = '0';
            // println!("result = 0, carry = 0");
        } else if count_1 == 1 {
            result.push('1');
            carry = '0';
            // println!("result = 1, carry = 0\n");
        } else if count_1 == 2 {
            result.push('0');
            carry = '1';
            // println!("result = 0, carry = 1\n");
        } else {
            result.push('1');
            carry = '1';
            // println!("result = 1, carry = 1\n");
        }
    }
    if carry == '1' {
        result.push('1');
    }
    println!("Output: \"{}\"\n", result.chars().rev().collect::<String>());
    // println!();
    result.chars().rev().collect::<String>()
}

fn main() {
    println!("Adding binaries:\n");

    let test_cases = vec![
        ("11".to_string(), "1".to_string()),
        ("1010".to_string(), "1011".to_string()),
    ];

    for (a, b) in test_cases {
        add_binary(a, b);
    }
}
