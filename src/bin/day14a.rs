use std::char;

fn main() {
    let mut recipes = vec![3, 7];
    let mut i: usize = 0;
    let mut j: usize = 1;

    let input = 768071;

    while recipes.len() < 10 + input {
        let sum = recipes[i] + recipes[j];
        if sum < 10 {
            recipes.push(sum);
        } else {
            recipes.push(1);
            recipes.push(sum - 10);
        }
        i = (i + recipes[i] + 1) % recipes.len();
        j = (j + recipes[j] + 1) % recipes.len();
    }

    let output: String = recipes[input..(input + 10)]
        .iter()
        .map(|&d| char::from_digit(d as u32, 10).unwrap())
        .collect();
    println!("{}", output);
}
