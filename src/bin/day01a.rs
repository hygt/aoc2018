use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day01.txt").expect("Couldn't read file");
    let freq = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .into_iter()
        .fold(0, |acc, x| acc + x);

    println!("{}", freq);
}
