use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq, Hash)]
struct Point {
    id: i8,
    x: i32,
    y: i32,
}

// at first glance every coordinate is < 400
const DIM: i32 = 400;

fn main() {
    let input = fs::read_to_string("resources/day06.txt").expect("Couldn't read file");

    let mut left = DIM;
    let mut right = 0;
    let mut top = DIM;
    let mut bottom = 0;
    let mut id = 0i8;

    let points: Vec<Point> = input
        .lines()
        .into_iter()
        .map(|line| {
            let tokens: Vec<i32> = line
                .split(", ")
                .map(|i| i.parse::<i32>().unwrap())
                .collect();
            let x = tokens[0];
            let y = tokens[1];
            left = left.min(x);
            right = right.max(x);
            top = top.min(y);
            bottom = bottom.max(y);
            id += 1;
            Point { id, x, y }
        })
        .collect();

    let mut board = vec![vec![0i8; DIM as usize]; DIM as usize];
    for i in left..=right {
        for j in top..=bottom {
            id = shortest_distance(i, j, &points);
            board[i as usize][j as usize] = id;
        }
    }

    // walk the borders, flag these points as their respective area is infinite
    let mut infinite: HashSet<i8> = HashSet::new();
    for i in left..=right {
        // top
        let id = board[i as usize][top as usize];
        infinite.insert(id);
        // bottom
        let id = board[i as usize][bottom as usize];
        infinite.insert(id);
    }
    for j in top..=bottom {
        // left
        let id = board[left as usize][j as usize];
        infinite.insert(id);
        // right
        let id = board[right as usize][j as usize];
        infinite.insert(id);
    }

    // count occurrences
    let mut count = vec![0u32; points.len()];
    for i in left..=right {
        for j in top..=bottom {
            let id = board[i as usize][j as usize];
            if id != -1 && !infinite.contains(&id) {
                count[id as usize] += 1;
            }
        }
    }

    let max = count.iter().max().unwrap();
    println!("Max area: {}", max);
}

/// Returns the id of the closest Point or -1 if there are more than one.
fn shortest_distance(x: i32, y: i32, points: &Vec<Point>) -> i8 {
    let mut set: HashSet<&Point> = HashSet::new();
    let mut min = DIM;
    for p in points {
        let d = (p.x - x).abs() + (p.y - y).abs();
        if d < min {
            set.clear();
            set.insert(p);
            min = d;
            if d == 0 {
                break;
            } // small optimization, we're on a point itself
        } else if d == min {
            set.insert(p);
        }
    }
    if set.len() == 1 {
        set.iter().nth(0).unwrap().id
    } else {
        -1
    }
}
