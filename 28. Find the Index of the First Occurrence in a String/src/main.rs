pub fn str_str(haystack: String, needle: String) -> i32 {
    println!("Input: haystack = {:?}, needle = {}", haystack, needle);

    if haystack.contains(&needle) {
        println!("Output: {:?}", haystack.find(&needle).unwrap());
        haystack.find(&needle).unwrap() as i32
    } else {
        println!("Output: {}", -1);
        -1
    }
}

fn main() {
    println!("Finding the index of the first occurrence in a string:\n");

    let test_cases = vec![("sadbutsad", "sad"), ("leetcode", "leeto")];

    for (haystack, needle) in test_cases {
        str_str(haystack.to_string(), needle.to_string());
    }
}
