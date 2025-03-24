// ---> Versão O(n^2) no pior caso <---
// Itera 2 vezes sobre o vetor, verificando se a soma de dois elementos é igual ao target caso os elementos não tenham o mesmo índice.
// Se for, retorna os índices dos elementos que somados resultam no target.
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut return_vec = Vec::new();
//     for (idx1, num1) in nums.clone().iter().enumerate() {
//         for (idx2, num2) in nums.iter().enumerate() {
//             if idx1 != idx2 {
//                 // println!("Index 1: {}", idx1);
//                 // println!("Index 2: {}\n", idx2);
//                 if num1 + num2 == target {
//                     // println!(
//                     //     "Found: {} (index {}) + {} (index {}) = {}\n",
//                     //     num1, idx1, num2, idx2, target
//                     // );
//                     return_vec.push(idx1 as i32);
//                     return_vec.push(idx2 as i32);
//                     return return_vec;
//                 }
//             }
//             // else {
//             //     println!("Equal!\n");
//             // }
//         }
//     }
//     return_vec
// }

// ---> Versão O(n) no pior caso <---
// Itera uma vez sobre o vetor, verificando se o complemento do elemento atual está no HashMap.
// Se estiver, retorna o índice do complemento e o índice atual.
// Se não estiver, insere o elemento atual no HashMap.
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new(); // Chave: valor, Valor: índice
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&idx_complement) = map.get(&complement) {
            return vec![idx_complement as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output = two_sum(nums, target);
    println!("Output: {:?}", output);

    let nums = vec![3, 2, 4];
    let target = 6;
    let output = two_sum(nums, target);
    println!("Output: {:?}", output);

    let nums = vec![3, 3];
    let target = 6;
    let output = two_sum(nums, target);
    println!("Output: {:?}", output);
}
