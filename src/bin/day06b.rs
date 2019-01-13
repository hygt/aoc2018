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
                .into_iter()
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

    // luckily the area is contiguous
    let mut count = 0;
    for i in left..=right {
        for j in top..=bottom {
            if distances_sum(i, j, &points) < 10_000 {
                count += 1;
            }
        }
    }
    println!("Max area: {}", count);
}

/// Returns the sum of distances between this position and all points.
fn distances_sum(x: i32, y: i32, points: &Vec<Point>) -> i32 {
    points
        .iter()
        .fold(0, |acc, p| acc + (p.x - x).abs() + (p.y - y).abs())
}
