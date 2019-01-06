extern crate edit_distance;

use edit_distance::edit_distance;
use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day02.txt").expect("Couldn't read file");
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();

    for i in 1..lines.len() {
        if edit_distance(lines[i - 1], lines[i]) == 1 {
            print(lines[i - 1], lines[i]);
        }
    }
}

fn print(a: &str, b: &str) {
    let mut string = String::new();
    let mut achars = a.chars();
    let mut bchars = b.chars();
    for _ in 0..a.len() {
        let ai = achars.next();
        let bi = bchars.next();
        if ai == bi {
            string.push(ai.unwrap());
        }
    }
    println!("{}", string);
}
