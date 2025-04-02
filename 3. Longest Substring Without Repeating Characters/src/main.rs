pub fn length_of_longest_substring(s: String) -> i32 {
    let mut substrings: Vec<String> = vec![];
    let mut remaining_substring = s;

    while !remaining_substring.is_empty() {
        substrings.push(get_substring(&remaining_substring));
        remaining_substring.remove(0);
    }

    // println!("Substrings: {:?}", substrings);

    substrings.sort_by_key(|a| a.len());

    // println!("Sorted substrings by lenght: {:?}", substrings);
    // println!("Longest substring: {:?}", substrings.last());
    // println!(
    //     "Length of longest substring: {:?}",
    //     substrings.last().map(|s| s.len()).unwrap_or(0)
    // );

    substrings.last().map(|s| s.len()).unwrap_or(0) as i32
}

fn get_substring(s: &str) -> String {
    let mut substring = String::new();
    for char in s.chars() {
        if !substring.contains(char) {
            substring.push(char);
        } else {
            break;
        }
    }
    substring
}

fn main() {
    let s = String::from("abcabcbb");
    let result = length_of_longest_substring(s);
    println!("Output: {}", result);

    let s = String::from("bbbbb");
    let result = length_of_longest_substring(s);
    println!("Output: {}", result);

    let s = String::from("pwwkew");
    let result = length_of_longest_substring(s);
    println!("Output: {}", result);

    let s = String::from("dvdf");
    let result = length_of_longest_substring(s);
    println!("Output: {}", result);
}
