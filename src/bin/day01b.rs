use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day01.txt").expect("Couldn't read file");
    let mut set: HashSet<i32> = HashSet::new();
    let mut sum: i32 = 0;
    set.insert(0);
    let freqs: Vec<i32> = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    find_freq(&mut sum, &mut set, &freqs);
    println!("First frequency reached twice: {}", sum);
}

fn find_freq(sum: &mut i32, set: &mut HashSet<i32>, freqs: &[i32]) {
    let freq = freqs.iter().find(|&x| {
        *sum += x;
        if set.contains(sum) {
            true
        } else {
            set.insert(*sum);
            false
        }
    });
    if freq.is_none() {
        find_freq(sum, set, freqs);
    }
}
