extern crate regex;

use regex::Regex;
use std::fs;

const DIM: usize = 1024;
const MAX_ID: usize = 1295;

fn main() {
    let input = fs::read_to_string("resources/day03.txt").expect("Couldn't read file");

    let lines: Vec<&str> = input.lines().collect();
    let re = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut ids = vec![false; MAX_ID];
    let mut board = vec![vec![0u32; DIM]; DIM];
    let mut id: u32 = 0;
    for &line in lines.iter() {
        id += 1;
        let groups = re.captures(line).unwrap();
        let &x = &groups[1].parse::<usize>().unwrap();
        let &y = &groups[2].parse::<usize>().unwrap();
        let &w = &groups[3].parse::<usize>().unwrap();
        let &h = &groups[4].parse::<usize>().unwrap();
        for i in x..(x + w) {
            for j in y..(y + h) {
                let cell = board[i][j];
                if cell == 0 {
                    board[i][j] = id;
                } else {
                    ids[(id - 1) as usize] = true;
                    ids[(cell - 1) as usize] = true;
                }
            }
        }
    }
    for i in 0..MAX_ID {
        if !ids[i] {
            println!("{}", (i + 1));
        }
    }
}
