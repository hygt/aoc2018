// With 7108200 marbles a Vec won't do.
extern crate linked_list;

use linked_list::*;

const PLAYERS: usize = 413;
const MARBLES: u32 = 7_108_200;

fn main() {
    let mut marbles: LinkedList<u32> = LinkedList::new();
    marbles.push_back(0);
    let cursor = &mut marbles.cursor();
    let mut players = vec![0u32; PLAYERS];

    let mut p = 0;
    let mut len = 1 as usize;
    for m in 1..=MARBLES {
        if m % 23 == 0 {
            back(cursor, 7, len);
            players[p] += m + cursor.remove().unwrap();
            len -= 1;
        } else {
            fwd(cursor, 2);
            cursor.insert(m);
            len += 1;
        }
        p = (p + 1) % PLAYERS;
    }

    println!("high score: {}", players.iter().max().unwrap())
}

fn fwd(cur: &mut Cursor<u32>, by: u32) {
    for _ in 0..by {
        if cur.peek_next().is_none() {
            cur.reset();
        }
        cur.next();
    }
}

fn back(cur: &mut Cursor<u32>, by: u32, len: usize) {
    for _ in 0..by {
        if cur.peek_prev().is_none() {
            cur.seek_forward(len);
        }
        cur.prev();
    }
}
