extern crate regex;

use regex::Regex;
use std::fs;

const DIM: usize = 1024;

fn main() {
    let input = fs::read_to_string("resources/day03.txt").expect("Couldn't read file");

    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut board = vec![vec![0u8; DIM]; DIM];

    for &line in lines.iter() {
        let groups = re.captures(line).unwrap();
        let &x = &groups[1].parse::<usize>().unwrap();
        let &y = &groups[2].parse::<usize>().unwrap();
        let &w = &groups[3].parse::<usize>().unwrap();
        let &h = &groups[4].parse::<usize>().unwrap();
        for i in x..(x + w) {
            for j in y..(y + h) {
                board[i][j] += 1;
            }
        }
    }
    let mut sum: u32 = 0;
    for i in 0..DIM {
        for j in 0..DIM {
            if board[i][j] > 1 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
