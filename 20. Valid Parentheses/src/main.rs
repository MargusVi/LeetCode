pub fn is_valid(s: String) -> bool {
    let mut open_order = vec![];

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => open_order.push(c),
            ')' | ']' | '}' => {
                let last_open = match open_order.last() {
                    Some(bracket) => *bracket,
                    None => return false,
                };

                let matches = match c {
                    ')' => last_open == '(',
                    ']' => last_open == '[',
                    '}' => last_open == '{',
                    _ => unreachable!(),
                };

                if matches {
                    open_order.pop();
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }

    open_order.is_empty()
}

fn main() {
    println!("Are the parenthesis in the string valid?\n");

    let inputs = vec![
        "()".to_string(),
        "()[]{}".to_string(),
        "(]".to_string(),
        "([])".to_string(),
    ];

    for input in inputs {
        println!("Input: {}\tOutput: {}", input.clone(), is_valid(input));
    }
}
