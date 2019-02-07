const GENERATIONS: u64 = 50_000_000_000;

fn main() {
    // padded with some pots on both ends
    let mut pots = parse(".....##.###.......#..#.##..#####...#...#######....##.##.##.##..#.#.##########...##.##..##.##...####..####............................................................");
    let len = pots.len();
    let offset: i32 = -5;
    let mut shift: i32 = 0;

    let grow: Vec<Vec<bool>> = [
        "#.#.#", "..###", ".#..#", "####.", "###..", ".#.#.", "#...#", "..#.#", "#..#.", ".#...",
        ".#.##", "#.##.", "...##", "###.#", "#####",
    ]
    .iter()
    .map(|l| parse(l))
    .collect();

    let die: Vec<Vec<bool>> = [
        ".##..", "#.#..", "..#..", "#....", "....#", "##..#", "##...", "#..##", ".##.#", ".####",
        ".###.", "..##.", "##.#.", "...#.", ".....", "##.##", "#.###",
    ]
    .iter()
    .map(|l| parse(l))
    .collect();

    let mut by = 0;
    let mut iteration = 0u64;
    loop {
        iteration += 1;
        let mut next = vec![false; len];
        for i in 0..(len - 5) {
            let slice = pots[i..(i + 5)].to_vec();
            next[i + 2] = if grow.contains(&slice) {
                true
            } else if die.contains(&slice) {
                false
            } else {
                slice[i + 2]
            };
        }

        for (i, &b) in next.iter().enumerate() {
            if b {
                by = i as i32 + offset;
                break;
            }
        }
        shift += by;
        shift_pots(&mut next, len as i32, by);

        if pots == next {
            break;
        } else {
            pots = next;
        }
        print(pots.clone());
    }

    let mut sum = 0u64;
    for (i, &b) in pots.iter().enumerate() {
        if b {
            sum += i as u64;
            sum += (offset + shift) as u64;
            sum += GENERATIONS - iteration;
        }
    }
    println!("{}", sum);
}

fn parse(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn print(line: Vec<bool>) {
    let s: String = line.iter().map(|&b| if b { '#' } else { '.' }).collect();
    println!("{}", s);
}

fn shift_pots(vec: &mut Vec<bool>, len: i32, by: i32) {
    if by > 0 {
        for i in 0..(len - by) {
            vec[i as usize] = vec[(i + by) as usize];
        }
        for i in (len - by)..len {
            vec[i as usize] = false;
        }
    } else if by < 0 {
        for i in 0..(len + by) {
            vec[(len - i - 1) as usize] = vec[(len - i - 1 + by) as usize];
        }
        for i in (len + by)..len {
            vec[(len - i - 1) as usize] = false;
        }
    }
}
