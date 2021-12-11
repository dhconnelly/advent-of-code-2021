use std::collections::{HashMap, VecDeque};

type Pt = (i32, i32);
type Grid = HashMap<Pt, u8>;

static DIRS: [Pt; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn nbrs((row, col): Pt) -> impl Iterator<Item = Pt> {
    DIRS.iter().map(move |(drow, dcol)| (row + drow, col + dcol))
}

fn step(grid: &mut Grid) -> usize {
    let mut q = VecDeque::from_iter(grid.keys().copied());
    while let Some(pt) = q.pop_front() {
        if grid.contains_key(&pt) {
            let val = grid.get_mut(&pt).unwrap();
            *val += 1;
            if *val == 10 {
                q.extend(nbrs(pt));
            }
        }
    }
    grid.values_mut().fold(0, |mut acc, val| {
        if *val > 9 {
            *val = 0;
            acc += 1
        }
        acc
    })
}

fn part1(mut g: Grid) -> usize {
    (0..100).fold(0, |acc, _| acc + step(&mut g))
}

fn parse(s: &str) -> Grid {
    let mut g = Grid::new();
    for (row, line) in s.lines().enumerate() {
        for (col, ch) in line.bytes().enumerate() {
            g.insert((row as i32, col as i32), ch - b'0');
        }
    }
    g
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = parse(&text);
    println!("{}", part1(grid.clone()));
}
