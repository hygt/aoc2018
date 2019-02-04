fn main() {
    let mut grid = vec![vec![0; 300]; 300];
    let serial_number = 4455;
    for x in 0usize..300 {
        for y in 0usize..300 {
            let rack_id = x + 11;
            let start_level = rack_id * (y + 1);
            let power_level = (start_level + serial_number) * rack_id;
            let hundreds_digit = (power_level / 100) % 10;
            let level = (hundreds_digit as i32) - 5;
            grid[y][x] = level;
        }
    }
    println!("{:?}", find_square(&grid, 3));

    let max = (1..=300)
        .map(|s| find_square(&grid, s))
        .max_by_key(|s| s.total)
        .unwrap();
    println!("{:?}", max);
}

#[derive(Debug)]
struct Square {
    size: usize,
    x: usize,
    y: usize,
    total: i32,
}

fn find_square(grid: &[Vec<i32>], size: usize) -> Square {
    let boundary = 301 - size;
    let mut squares = vec![vec![0; boundary]; boundary];
    for x in 0usize..boundary {
        for y in 0usize..boundary {
            if x == 0 || y == 0 {
                let mut sum = 0;
                for i in x..(x + size) {
                    for j in y..(y + size) {
                        sum += grid[j][i];
                    }
                }
                squares[y][x] = sum;
            } else {
                let mut sum = squares[y - 1][x - 1];
                for i in y..(y + size - 1) {
                    sum -= grid[i][x - 1];
                    sum += grid[i][x + size - 1];
                }
                for i in x..(x + size) {
                    sum -= grid[y - 1][i - 1];
                    sum += grid[y + size - 1][i];
                }
                squares[y][x] = sum;
            }
        }
    }

    let mut max = i32::min_value();
    let mut i = 0;
    let mut j = 0;
    for x in 0usize..boundary {
        for y in 0usize..boundary {
            if squares[y][x] > max {
                max = squares[y][x];
                i = x;
                j = y;
            }
        }
    }
    Square {
        size,
        x: i + 1,
        y: j + 1,
        total: max,
    }
}
