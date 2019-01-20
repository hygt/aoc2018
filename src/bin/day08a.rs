use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day08.txt").expect("Couldn't read file");
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut done = false;
    let mut sum = 0;
    while !done {
        let mut i = 0usize;
        while i < nums.len() {
            let num = nums[i];
            if num == 0 {
                if i == 0 {
                    done = true;
                } else {
                    // go back to the previous node and decrease children header
                    let mut j = i - 1;
                    while nums[j] == -1 {
                        j -= 1;
                    }
                    nums[j - 1] -= 1;
                }
                // mark children header cell
                nums[i] = -1;
                i += 1;
                // read meta header and mark cell
                let meta = nums[i];
                nums[i] = -1;
                // skip previously marked cells
                while nums[i] == -1 {
                    i += 1;
                }
                // add metadata to sum and mark cells
                for _ in 0..meta {
                    sum += nums[i];
                    nums[i] = -1;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    println!("{}", sum);
}
