use std::collections::*;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    children: Vec<i32>,
    meta: Vec<i32>,
}

impl Node {
    fn new() -> Node {
        Node {
            children: Vec::new(),
            meta: Vec::new(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("resources/day08.txt").expect("Couldn't read file");
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut tree: HashMap<i32, Node> = HashMap::new();
    let mut done = false;
    while !done {
        let mut i = 0usize;
        while i < nums.len() {
            let num = nums[i];
            if num == 0 {
                let child_pos = i as i32;
                if i == 0 {
                    done = true;
                } else {
                    // go back to the previous node and decrease children header
                    let mut j = i - 1;
                    while nums[j] == -1 {
                        j -= 1;
                    }
                    j -= 1;
                    nums[j] -= 1;
                    tree.entry(j as i32)
                        .or_insert(Node::new())
                        .children
                        .push(child_pos);
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
                // read metadata and mark cells
                let mut slice: Vec<i32> = Vec::new();
                for _ in 0..meta {
                    slice.push(nums[i]);
                    nums[i] = -1;
                    i += 1;
                }
                tree.entry(child_pos).or_insert(Node::new()).meta = slice;
            } else {
                i += 1;
            }
        }
    }

    println!("{}", recurse(&0, &tree));
}

fn recurse(id: &i32, tree: &HashMap<i32, Node>) -> i32 {
    let node = tree.get(id).unwrap();
    if node.children.len() == 0 {
        node.meta.iter().sum()
    } else {
        let mut sum = 0;
        for &i in &node.meta {
            if let Some(child) = node.children.get((i - 1) as usize) {
                sum += recurse(child, tree);
            }
        }
        sum
    }
}
