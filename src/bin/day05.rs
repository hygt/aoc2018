use std::fs;
use std::str::Chars;

fn main() {
    let input = fs::read_to_string("resources/day05.txt").expect("Couldn't read file");
    let ref chars = input.chars();

    // part one
    println!("Final size: {}", process(chars, None));

    // part two
    let mut min = input.len();
    for c in (b'a'..=b'z').map(char::from) {
        min = min.min(process(chars, Some(c)));
    }
    println!("Shortest size: {}", min);
}

fn process(chars: &Chars, skip: Option<char>) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for c in chars.clone().filter(|&x| filter(skip, x)) {
        if let Some(&peek) = stack.last() {
            if react(peek, c) {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }
    stack.len()
}

fn filter(skip: Option<char>, c: char) -> bool {
    match skip {
        None => true,
        Some(s) => s != c && !react(s, c),
    }
}

/// An uppercase letter reacts in presence of its lowercase counterpart,
/// i.e. if their ASCII codepoints are 32 apart.
fn react(a: char, b: char) -> bool {
    let diff = (a as i8) - (b as i8);
    diff == 32 || diff == -32
}
