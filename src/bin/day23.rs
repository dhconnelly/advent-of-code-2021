use std::collections::BTreeSet;
use std::collections::VecDeque;
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

impl fmt::Debug for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        };
        write!(f, "{}", ch)
    }
}

impl Amphipod {
    fn step_cost(self) -> i32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn belongs_in_room(self, room: u8) -> bool {
        match self {
            Amphipod::A => room == 0,
            Amphipod::B => room == 1,
            Amphipod::C => room == 2,
            Amphipod::D => room == 3,
        }
    }
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

type Tile = Option<Amphipod>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pos {
    Hallway(u8),
    Room(u8, u8),
}

#[derive(Debug)]
struct Burrow {
    pods: Vec<Pos>,
    hallway: Vec<Tile>,
    rooms: Vec<Vec<Tile>>,
}

impl Burrow {
    fn at(&self, pos: Pos) -> Tile {
        match pos {
            Pos::Hallway(i) => self.hallway[i as usize],
            Pos::Room(i, j) => self.rooms[i as usize][j as usize],
        }
    }

    fn is_done(&self) -> bool {
        let all_are = |pod, room: &[Tile]| room.iter().all(|&t| t == Some(pod));
        all_are(Amphipod::A, &self.rooms[0])
            && all_are(Amphipod::B, &self.rooms[1])
            && all_are(Amphipod::C, &self.rooms[2])
            && all_are(Amphipod::D, &self.rooms[3])
    }
}

impl TryFrom<Vec<u8>> for Burrow {
    type Error = ParseError;
    fn try_from(buf: Vec<u8>) -> Result<Self, Self::Error> {
        let grid: Vec<&[u8]> = buf.split(|&b| b == b'\n').collect();
        let mut rooms = vec![vec![None; 2]; 4];
        let mut pods = vec![];
        for room in 0..4 {
            for depth in 0..2 {
                let pod = Amphipod::try_from(grid[2 + depth][3 + room * 2])?;
                rooms[room][depth] = Some(pod);
                pods.push(Pos::Room(room as u8, depth as u8));
            }
        }
        let hallway = vec![None; 11];
        Ok(Burrow { hallway, rooms, pods })
    }
}

fn can_visit(burrow: &Burrow, from: Pos, dest: Pos, pod: Amphipod) -> bool {
    if burrow.at(dest).is_some() {
        return false;
    }
    // assume we're only ever checking tiles adjacent to the current one
    match dest {
        Pos::Hallway(i) => !matches!(from, Pos::Hallway(_)),
        Pos::Room(_, _) if matches!(from, Pos::Room(_, _)) => true,
        Pos::Room(room, _) if !pod.belongs_in_room(room) => false,
        Pos::Room(room, _) => burrow.rooms[room as usize]
            .iter()
            .all(|pod2| pod2.is_none() || pod2.unwrap() == pod),
    }
}

fn can_stop(burrow: &Burrow, pos: Pos) -> bool {
    match pos {
        Pos::Room(_, _) => true,
        Pos::Hallway(i) if i < 2 || i > 8 || (i % 2 != 0) => true,
        _ => false,
    }
}

fn nbrs(burrow: &Burrow, pos: Pos) -> [Option<Pos>; 3] {
    let mut nbrs = [None; 3];
    let mut j = 0;
    let mut push = |pos| {
        nbrs[j] = Some(pos);
        j += 1;
    };

    match pos {
        Pos::Hallway(i) => {
            if i > 0 {
                push(Pos::Hallway(i - 1));
            }
            if (i as usize) < burrow.hallway.len() - 1 {
                push(Pos::Hallway(i + 1));
            }
            if i >= 2 && i < 9 && (i % 2 == 0) {
                let room = (i - 2) / 2;
                push(Pos::Room(room, 0));
            }
        }

        Pos::Room(room, depth) => {
            if depth == 0 {
                push(Pos::Hallway(room * 2 + 2));
            } else {
                push(Pos::Room(room, depth - 1));
            }
            if (depth as usize) < burrow.rooms[room as usize].len() - 1 {
                push(Pos::Room(room, depth + 1));
            }
        }
    }

    nbrs
}

fn destinations(burrow: &Burrow, from: Pos) -> Option<Vec<(Pos, i32)>> {
    let pod = burrow.at(from)?;
    let mut destinations = vec![];

    // bfs
    let mut q = VecDeque::new();
    q.push_back((from, 0));
    let mut v = BTreeSet::new();
    v.insert(from);

    while let Some((pos, cost)) = q.pop_front() {
        if pos != from && can_stop(burrow, pos) {
            destinations.push((pos, cost));
        }
        for nbr in nbrs(burrow, pos).into_iter().filter_map(|p| p) {
            if !v.contains(&nbr) && can_visit(burrow, from, nbr, pod) {
                v.insert(nbr);
                q.push_back((nbr, cost + pod.step_cost()));
            }
        }
    }

    Some(destinations)
}

fn main() {
    let path = env::args().nth(1).expect("missing input path");
    let data = fs::read(&path).unwrap();
    let burrow = Burrow::try_from(data).unwrap();
    println!("{:?}", destinations(&burrow, Pos::Room(2, 0)));
}
