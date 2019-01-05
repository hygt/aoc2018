extern crate bit_set;

use bit_set::BitSet;
use std::fs;

// Using a BitSet that was removed from the standard lib long ago.
// It somehow appears ~5x slower than the other implementation with a plain HashSet.
fn main() {
    let input = fs::read_to_string("resources/day01.txt").expect("Couldn't read file");
    let mut set = BitSet::new();
    set.insert(enc(0));
    let mut sum: i32 = 0;
    let freqs: Vec<i32> = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    let mut freq: Option<&i32> = None;
    while freq.is_none() {
        freq = freqs.iter().find(|&&x| {
            sum += x;
            if set.contains(enc(sum)) {
                true
            } else {
                set.insert(enc(sum));
                false
            }
        });
    }
    match freq {
        Some(_) => println!("First frequency reached twice: {}", sum),
        None => println!("No frequency was reached twice."),
    }
}

fn enc(i: i32) -> usize {
    let mut offset = i32::max_value() as i64;
    offset += i as i64;
    offset as usize
}
