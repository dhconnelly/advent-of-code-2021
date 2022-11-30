use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
struct ParseError;

#[derive(Clone, Copy, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl TryFrom<u8> for Amphipod {
    type Error = ParseError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'A' => Ok(Amphipod::A),
            b'B' => Ok(Amphipod::B),
            b'C' => Ok(Amphipod::C),
            b'D' => Ok(Amphipod::D),
            _ => Err(ParseError),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Amphipod(Amphipod),
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Tile::Empty => '.',
            Tile::Amphipod(Amphipod::A) => 'A',
            Tile::Amphipod(Amphipod::B) => 'B',
            Tile::Amphipod(Amphipod::C) => 'C',
            Tile::Amphipod(Amphipod::D) => 'D',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug)]
struct Burrow {
    hallway: Vec<Tile>,
    rooms: Vec<Vec<Tile>>,
}

impl TryFrom<Vec<u8>> for Burrow {
    type Error = ParseError;
    fn try_from(buf: Vec<u8>) -> Result<Self, Self::Error> {
        let grid: Vec<&[u8]> = buf.split(|&b| b == b'\n').collect();
        let mut rooms = vec![vec![Tile::Empty; 2]; 4];
        for room in 0..4 {
            for depth in 0..2 {
                rooms[room][depth] = Tile::Amphipod(Amphipod::try_from(
                    grid[2 + depth][3 + room * 2],
                )?);
            }
        }
        Ok(Burrow { hallway: Vec::new(), rooms })
    }
}

fn main() {
    let path = env::args().nth(1).expect("missing input path");
    let data = fs::read(&path).unwrap();
    let burrow = Burrow::try_from(data).unwrap();
    println!("{:?}", burrow);
}
