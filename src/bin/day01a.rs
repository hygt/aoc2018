use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day01.txt").expect("Couldn't read file");
    let freq: i32 = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .sum();

    println!("{}", freq);
}
