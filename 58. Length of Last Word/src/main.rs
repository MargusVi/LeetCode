pub fn length_of_last_word(s: String) -> i32 {
    println!("Input: s = {}", s);
    println!(
        "Output: {}\n",
        s.split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .len() as i32
    );
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .len() as i32
}

fn main() {
    println!("Lenght of the last word in a string:\n");

    let test_cases = vec![
        "Hello World",
        "   fly me   to   the moon  ",
        "luffy is still joyboy",
    ];

    for test_case in test_cases {
        length_of_last_word(test_case.to_string());
    }
}
