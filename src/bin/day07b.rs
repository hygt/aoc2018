use std::collections::*;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    inc: BTreeSet<char>,
    out: BTreeSet<char>,
}

fn main() {
    // input content was edited for convenient parsing
    let input = fs::read_to_string("resources/day07.txt").expect("Couldn't read file");

    let mut graph: HashMap<char, Vertex> = HashMap::new();
    let lines: Vec<&str> = input.lines().into_iter().collect();

    for line in lines {
        let tokens: Vec<char> = line
            .split(" ")
            .into_iter()
            .map(|c| char::from_str(c).unwrap())
            .collect();
        let from = tokens[0];
        let to = tokens[1];
        // outgoing
        graph
            .entry(from)
            .or_insert(Vertex {
                inc: BTreeSet::new(),
                out: BTreeSet::new(),
            })
            .out
            .insert(to);
        // incoming
        graph
            .entry(to)
            .or_insert(Vertex {
                inc: BTreeSet::new(),
                out: BTreeSet::new(),
            })
            .inc
            .insert(from);
    }
    println!("{}", topo_sort(&graph));
}

fn topo_sort(graph: &HashMap<char, Vertex>) -> i32 {
    let mut visited: BTreeSet<char> = BTreeSet::new();
    let mut to_visit: BTreeSet<char> = BTreeSet::new();
    let mut pipeline: HashMap<char, i32> = HashMap::new();
    let mut tick = -1;
    for (c, v) in graph.iter() {
        if v.inc.is_empty() {
            pipeline.insert(*c, time(*c, tick));
        }
    }

    while !to_visit.is_empty() || !pipeline.is_empty() {
        tick += 1;
        for (c, t) in pipeline.clone() {
            if t == tick {
                println!("Node: {} tick: {}", c, tick);
                visited.insert(c);
                pipeline.remove(&c);

                let v = graph.get(&c).unwrap();
                for out in v.out.iter() {
                    if !visited.contains(out) {
                        to_visit.insert(*out);
                    }
                }

                let copy = to_visit.clone();
                for c in copy.iter() {
                    let v = graph.get(&c).unwrap();
                    let copy = visited.clone();
                    let remaining: Vec<&char> = v.inc.difference(&copy).collect();
                    if remaining.is_empty() && pipeline.len() < 5 {
                        pipeline.insert(*c, time(*c, tick));
                        to_visit.remove(c);
                    }
                }
            }
        }
    }
    tick + 1
}

fn time(c: char, tick: i32) -> i32 {
    tick + 61 + (c as i32) - ('A' as i32)
}
