use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use Direction::*;
use Turn::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y == other.y {
            self.x.partial_cmp(&other.x)
        } else {
            self.y.partial_cmp(&other.y)
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    D,
    R,
    L,
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    dir: Direction,
    turn: Turn,
}

fn main() {
    let input = fs::read_to_string("resources/day13.txt").expect("Couldn't read file");

    let mut board: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let h = board.len();
    let w = board.first().unwrap().len();

    let mut carts: BTreeMap<Point, Cart> = BTreeMap::new();

    // initialize
    for y in 0..h {
        for x in 0..w {
            match board[y][x] {
                '^' => {
                    carts.insert(Point { x, y }, Cart { dir: U, turn: Left });
                    board[y][x] = '|';
                }
                'v' => {
                    carts.insert(Point { x, y }, Cart { dir: D, turn: Left });
                    board[y][x] = '|';
                }
                '>' => {
                    carts.insert(Point { x, y }, Cart { dir: R, turn: Left });
                    board[y][x] = '-';
                }
                '<' => {
                    carts.insert(Point { x, y }, Cart { dir: L, turn: Left });
                    board[y][x] = '-';
                }
                _ => (),
            }
        }
    }

    while carts.len() > 1 {
        carts.clone().iter().for_each(|(&point, cart)| {
            if carts.contains_key(&point) {
                let (np, nc) = next(point, cart.dir, cart.turn, board[point.y][point.x]);
                carts.remove(&point);
                if carts.contains_key(&np) {
                    println!("Collision: {:?} # carts: {}", np, carts.len());
                    carts.remove(&np);
                    if carts.len() == 1 {
                        println!("Last cart: {:?}", carts);
                    }
                } else {
                    carts.insert(np, nc);
                }
            }
        });
    }
}

fn next(point: Point, dir: Direction, turn: Turn, track: char) -> (Point, Cart) {
    match (dir, track, turn) {
        (U, '|', _) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart { dir: U, turn: turn },
        ),
        (U, '+', Straight) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart {
                dir: U,
                turn: Right,
            },
        ),
        (U, '/', _) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart { dir: R, turn: turn },
        ),
        (U, '+', Right) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart { dir: R, turn: Left },
        ),
        (U, '\\', _) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart { dir: L, turn: turn },
        ),
        (U, '+', Left) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart {
                dir: L,
                turn: Straight,
            },
        ),

        (D, '|', _) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart { dir: D, turn: turn },
        ),
        (D, '+', Straight) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart {
                dir: D,
                turn: Right,
            },
        ),
        (D, '/', _) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart { dir: L, turn: turn },
        ),
        (D, '+', Right) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart { dir: L, turn: Left },
        ),
        (D, '\\', _) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart { dir: R, turn: turn },
        ),
        (D, '+', Left) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart {
                dir: R,
                turn: Straight,
            },
        ),

        (L, '-', _) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart { dir: L, turn: turn },
        ),
        (L, '+', Straight) => (
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Cart {
                dir: L,
                turn: Right,
            },
        ),
        (L, '/', _) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart { dir: D, turn: turn },
        ),
        (L, '+', Left) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart {
                dir: D,
                turn: Straight,
            },
        ),
        (L, '\\', _) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart { dir: U, turn: turn },
        ),
        (L, '+', Right) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart { dir: U, turn: Left },
        ),

        (R, '-', _) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart { dir: R, turn: turn },
        ),
        (R, '+', Straight) => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Cart {
                dir: R,
                turn: Right,
            },
        ),
        (R, '/', _) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart { dir: U, turn: turn },
        ),
        (R, '+', Left) => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Cart {
                dir: U,
                turn: Straight,
            },
        ),
        (R, '\\', _) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart { dir: D, turn: turn },
        ),
        (R, '+', Right) => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Cart { dir: D, turn: Left },
        ),
        _ => unreachable!("Illegal move: {:?} {:?} {:?} {}", point, dir, turn, track),
    }
}
