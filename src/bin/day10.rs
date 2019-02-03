use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

fn main() {
    // input content was edited for convenient parsing
    let input = fs::read_to_string("resources/day10.txt").expect("Couldn't read file");

    let mut points: Vec<Point> = input
        .lines()
        .into_iter()
        .map(|line| {
            let tokens: Vec<i64> = line
                .split_whitespace()
                .map(|n| i64::from_str(n).unwrap())
                .collect();
            Point {
                x: tokens[0],
                y: tokens[1],
                dx: tokens[2],
                dy: tokens[3],
            }
        })
        .collect();

    let mut seconds = -1;
    let mut bounding_box: i64 = 15_000_000_000; // initial input is smaller than 110,000²
    let mut area: i64 = 0;
    while area <= bounding_box {
        let left = points.iter().min_by_key(|p| p.x).unwrap().x;
        let right = points.iter().max_by_key(|p| p.x).unwrap().x;
        let bottom = points.iter().min_by_key(|p| p.y).unwrap().y;
        let top = points.iter().max_by_key(|p| p.y).unwrap().y;
        let width = right - left;
        let height = top - bottom;
        area = width * height;
        if area < bounding_box {
            for point in points.iter_mut() {
                point.x += point.dx;
                point.y += point.dy;
            }
            bounding_box = area;
            seconds += 1;
        } else {
            // undo the last step
            for point in points.iter_mut() {
                point.x -= point.dx;
                point.y -= point.dy;
            }
            let mut bg = vec![vec![' '; width as usize]; height as usize];
            for point in points.iter() {
                bg[(point.y - bottom) as usize][(point.x - left) as usize] = '#';
            }
            for row in bg {
                let s: String = row.into_iter().collect();
                println!("{}", s);
            }
        }
    }
    println!("seconds: {}", seconds);
}
