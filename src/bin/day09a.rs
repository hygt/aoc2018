// 413 players; last marble is worth 71082 points; high score is 416424

const PLAYERS: usize = 413;
const MARBLES: i32 = 71082;

fn main() {
    let mut marbles = vec![0];
    marbles.reserve(MARBLES as usize); // we need less than that really
    let mut players = vec![0; PLAYERS];

    let mut current = 0;
    let mut p = 0;
    for m in 1..=MARBLES {
        let len = marbles.len() as i32;
        if m % 23 == 0 {
            let pos = current - 7;
            current = if pos >= 0 { pos } else { pos + len };
            players[p] += m + marbles.remove(current as usize);
        } else {
            let pos = current + 2;
            current = if pos <= len { pos } else { pos - len };
            marbles.insert(current as usize, m);
        }
        p = (p + 1) % PLAYERS;
    }

    println!("high score: {}", players.iter().max().unwrap())
}
