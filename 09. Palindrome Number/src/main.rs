// ---> Transformando o número em string <---
pub fn is_palindrome(x: i32) -> bool {
    let num_str = x.to_string();
    let reverse_num_str: String = num_str.chars().rev().collect();
    // println!("num_str: {}", num_str);
    // println!("reverse_num_str: {}", reverse_num_str);
    num_str == reverse_num_str
}

// // ---> Sem converter o número para string <---
// pub fn is_palindrome(x: i32) -> bool {
//     if x < 0 {
//         return false;
//     }

//     let mut reversed = 0;
//     let mut original = x;

//     while original > 0 {
//         reversed = reversed * 10 + original % 10;
//         original /= 10;
//     }

//     reversed == x
// }

fn main() {
    println!("Are the numbers palindromes?\n");

    let test_cases = vec![121, -121, 10];

    for &num in &test_cases {
        let output = is_palindrome(num);
        println!("Input: {}\tOutput: {}", num, output);
    }
}
