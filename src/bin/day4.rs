use std::collections::{HashMap, HashSet};

const ROWS: usize = 5;
const COLS: usize = 5;

type Idx = (usize, usize);

struct Board {
    tiles: HashMap<Idx, i32>,
    indices: HashMap<i32, Vec<Idx>>,
    marked: HashSet<Idx>,
    bingo: bool,
}

impl Board {
    fn sum_unmarked(&self) -> i32 {
        let val_of = |(idx, &n)| if self.marked.contains(idx) { 0 } else { n };
        self.tiles.iter().map(val_of).sum()
    }

    fn has_bingo(&self, (i, j): Idx) -> bool {
        (0..ROWS).all(|j| self.marked.contains(&(i, j)))
            || (0..COLS).all(|i| self.marked.contains(&(i, j)))
    }

    fn play(&mut self, value: i32) {
        if let Some(indices) = self.indices.get(&value) {
            self.marked.extend(indices.iter());
            if indices.iter().any(|idx| self.has_bingo(*idx)) {
                self.bingo = true;
            }
        }
    }
}

fn play_all(order: Vec<i32>, mut boards: Vec<Board>) -> Vec<i32> {
    let mut results = Vec::new();
    let mut removed = HashSet::new();
    for value in order {
        for i in 0..boards.len() {
            if removed.contains(&i) {
                continue;
            }
            boards[i].play(value);
            if boards[i].bingo {
                results.push(value * boards[i].sum_unmarked());
                removed.insert(i);
            }
        }
    }
    results
}

fn indices_table(tiles: &HashMap<Idx, i32>) -> HashMap<i32, Vec<Idx>> {
    tiles.iter().fold(HashMap::new(), |mut acc, (&idx, &val)| {
        acc.entry(val).or_insert(Vec::new()).push(idx);
        acc
    })
}

fn atoi(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

fn parse_board(s: &str) -> Board {
    let to_tile = |(i, s): (usize, &str)| ((i / COLS, i % COLS), atoi(s));
    let tiles = s.split_whitespace().enumerate().map(to_tile).collect();
    let indices = indices_table(&tiles);
    Board { tiles, indices, bingo: false, marked: HashSet::new() }
}

fn parse(s: &str) -> (Vec<i32>, Vec<Board>) {
    let mut segs = s.split("\n\n");
    let order = segs.next().unwrap().split(',').map(atoi).collect();
    let boards = segs.map(parse_board).collect();
    (order, boards)
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (order, boards) = parse(&text);
    let results = play_all(order, boards);
    println!("{}", results.first().unwrap());
    println!("{}", results.last().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_part1() {
        let (order, boards) = parse(INPUT);
        let results = play_all(order, boards);
        assert_eq!(4512, results[0]);
    }

    #[test]
    fn test_part2() {
        let (order, boards) = parse(INPUT);
        let results = play_all(order, boards);
        assert_eq!(1924, *results.last().unwrap());
    }
}
