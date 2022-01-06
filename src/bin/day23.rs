use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Fish(char),
}

impl Tile {
    fn from(ch: char) -> Option<Self> {
        use Tile::*;
        match ch {
            ' ' => None,
            '#' => Some(Wall),
            '.' => Some(Open),
            'A' | 'B' | 'C' | 'D' => Some(Fish(ch)),
            _ => panic!(),
        }
    }
}

type Pt = (i32, i32);

#[derive(Debug)]
struct State {
    tiles: HashMap<Pt, Tile>,
    fish: HashMap<char, Vec<Pt>>,
}

fn parse(s: &str) -> State {
    let parse_tile = |pt, ch| Tile::from(ch).map(|tile| (pt, tile));
    let tiles: HashMap<Pt, Tile> = s
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars().enumerate().filter_map(move |(col, ch)| {
                parse_tile((row as i32, col as i32), ch)
            })
        })
        .collect();
    let fish: HashMap<char, Vec<Pt>> =
        tiles.iter().fold(HashMap::new(), |mut acc, (&pt, &tile)| {
            if let Tile::Fish(ch) = tile {
                acc.entry(ch).or_default().push(pt);
            }
            acc
        });
    State { tiles, fish }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let state = parse(&text);
    println!("{:?}", state);
}
