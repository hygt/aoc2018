use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day02.txt").expect("Couldn't read file");
    let lines: Vec<&str> = input.lines().collect();

    println!("Checksum: {}", count(&lines, 2) * count(&lines, 3));
}

fn count(lines: &Vec<&str>, i: u32) -> u32 {
    let mut map: HashMap<char, u32> = HashMap::new();
    lines
        .iter()
        .filter(|&line| {
            for c in line.chars() {
                *map.entry(c).or_insert(0) += 1;
            }
            let found = map.iter().any(|(_k, &v)| v == i);
            map.clear();
            found
        })
        .count() as u32
}
