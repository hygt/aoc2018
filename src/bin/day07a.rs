use std::collections::*;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    inc: HashSet<char>,
    out: HashSet<char>,
}

fn main() {
    // input content was edited for convenient parsing
    let input = fs::read_to_string("resources/day07.txt").expect("Couldn't read file");

    let mut graph: HashMap<char, Vertex> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let tokens: Vec<char> = line
            .split_whitespace()
            .map(|c| char::from_str(c).unwrap())
            .collect();
        let from = tokens[0];
        let to = tokens[1];
        // outgoing
        graph
            .entry(from)
            .or_insert(Vertex {
                inc: HashSet::new(),
                out: HashSet::new(),
            })
            .out
            .insert(to);
        // incoming
        graph
            .entry(to)
            .or_insert(Vertex {
                inc: HashSet::new(),
                out: HashSet::new(),
            })
            .inc
            .insert(from);
    }
    let mut out = String::new();
    for c in topo_sort(&graph).iter() {
        out.push(*c);
    }
    println!("{}", out);
}

fn topo_sort(graph: &HashMap<char, Vertex>) -> Vec<char> {
    let mut visited: HashSet<char> = HashSet::new();
    let mut topo: Vec<char> = Vec::new();
    let mut to_visit: BTreeSet<char> = BTreeSet::new();
    for (c, v) in graph.iter() {
        if v.inc.is_empty() {
            to_visit.insert(*c);
        }
    }

    while !to_visit.is_empty() {
        let copy = to_visit.clone();
        let c = copy.iter().next().unwrap();
        to_visit.remove(c);
        let v = graph.get(c).unwrap();

        let copy = visited.clone();
        let remaining: Vec<&char> = v.inc.difference(&copy).collect();
        if remaining.is_empty() {
            topo.push(*c);
            visited.insert(*c);
            for out in v.out.iter() {
                if !visited.contains(out) {
                    to_visit.insert(*out);
                }
            }
        } else {
            for &inc in remaining.iter() {
                if !visited.contains(inc) {
                    to_visit.insert(*inc);
                }
            }
        }
    }
    topo
}
