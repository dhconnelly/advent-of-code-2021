use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fish(char);

impl Fish {
    fn idx(self) -> usize {
        self.0 as usize - 'A' as usize
    }

    fn from_idx(idx: usize) -> Self {
        Self(('A' as usize + idx) as u8 as char)
    }

    fn column(self) -> i64 {
        match self.0 {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            _ => 9,
        }
    }

    fn multiplier(self) -> i64 {
        match self.0 {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            _ => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Wall,
    Fish(Fish),
}

impl Tile {
    fn from(ch: char) -> Option<Self> {
        match ch {
            ' ' => None,
            '#' => Some(Tile::Wall),
            '.' => Some(Tile::Open),
            'A' | 'B' | 'C' | 'D' => Some(Tile::Fish(Fish(ch))),
            _ => panic!(),
        }
    }

    fn char(self) -> char {
        match self {
            Tile::Open => '.',
            Tile::Wall => '#',
            Tile::Fish(Fish(ch)) => ch,
        }
    }
}

type Pt = (i64, i64);
type Grid = BTreeMap<Pt, Tile>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    tiles: Grid,
    fish: [Vec<Pt>; 4],
}

impl State {
    fn digest(&self) -> String {
        self.tiles.iter().map(|(_, tile)| tile.char()).collect()
    }

    fn is_organized(&self) -> bool {
        self.fish[0].iter().all(|&(_, col)| col == 3)
            && self.fish[1].iter().all(|&(_, col)| col == 5)
            && self.fish[2].iter().all(|&(_, col)| col == 7)
            && self.fish[3].iter().all(|&(_, col)| col == 9)
    }
}

fn parse(s: &str) -> State {
    let parse_tile = |pt, ch| Tile::from(ch).map(|tile| (pt, tile));
    let tiles: Grid = s
        .lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars().enumerate().filter_map(move |(col, ch)| {
                parse_tile((row as i64, col as i64), ch)
            })
        })
        .collect();
    let fish = tiles.iter().fold(
        [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        |mut acc, (&pt, &tile)| {
            if let Tile::Fish(fish) = tile {
                acc[fish.idx()].push(pt);
            }
            acc
        },
    );
    State { tiles, fish }
}

fn is_own_room(fish: Fish, pos: &Pt) -> bool {
    (pos.0 == 2 || pos.0 == 3) && pos.1 == fish.column()
}

fn only_own_kind(tiles: &Grid, fish: Fish, pos: &Pt) -> bool {
    let col = pos.1;
    let tile1 = *tiles.get(&(2, col)).unwrap();
    let tile2 = *tiles.get(&(3, col)).unwrap();
    (tile1 == Tile::Open || tile1 == Tile::Fish(fish))
        && (tile2 == Tile::Open || tile2 == Tile::Fish(fish))
}

fn can_move(tiles: &Grid, fish: Fish, a: &Pt, b: &Pt) -> bool {
    // can't stay put
    if a == b {
        return false;
    }
    // ??? don't leave once you're in place
    if is_own_room(fish, a) && only_own_kind(tiles, fish, a) {
        return false;
    }
    // must be free
    if *tiles.get(b).unwrap() != Tile::Open {
        return false;
    }
    // can't stop in front of a room
    if b.0 == 1 && (b.1 == 3 || b.1 == 5 || b.1 == 7 || b.1 == 9) {
        return false;
    }
    // if in hallway: can only move into a room
    if a.0 == 1 && b.0 == 1 {
        return false;
    }
    // if moving into a room: must be its own and only have its own kind
    if b.0 == 2 || b.0 == 3 {
        if !is_own_room(fish, b) || !only_own_kind(tiles, fish, b) {
            return false;
        }
        if tiles.get(&(3, b.1)) == Some(&Tile::Open) && b.0 != 3 {
            return false;
        }
    }
    true
}

fn bfs(tiles: &BTreeMap<Pt, Tile>, a: Pt, b: Pt) -> Option<i64> {
    let mut q = VecDeque::new();
    q.push_back((a, 0));
    let mut v = HashSet::new();
    v.insert(a);
    while let Some(((r, c), dist)) = q.pop_front() {
        for nbr in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            if nbr == b {
                return Some(dist + 1);
            }
            if v.contains(&nbr) || tiles.get(&nbr) != Some(&Tile::Open) {
                continue;
            }
            v.insert(nbr);
            q.push_back((nbr, dist + 1));
        }
    }
    None
}

fn cost(tiles: &BTreeMap<Pt, Tile>, fish: Fish, a: &Pt, b: &Pt) -> Option<i64> {
    if !can_move(tiles, fish, a, b) {
        return None;
    }
    bfs(tiles, *a, *b).map(|dist| fish.multiplier() * dist)
}

type Move = (Fish, usize, Pt, Pt, i64);

fn next_moves(state: &State) -> Vec<Move> {
    let fish = state.fish.iter().enumerate();
    fish.map(|(j, xs)| (Fish::from_idx(j), xs))
        .flat_map(|(f, xs)| xs.iter().enumerate().map(move |(i, x)| (f, i, x)))
        .flat_map(|(f, i, a)| state.tiles.keys().map(move |b| (f, i, *a, *b)))
        .map(|(f, i, a, b)| (f, i, a, b, cost(&state.tiles, f, &a, &b)))
        .filter_map(|(fish, i, a, b, cost)| Some((fish, i, a, b, cost?)))
        .collect()
}

fn min_energy(
    state: &mut State,
    memo: &mut HashMap<String, i64>,
) -> Option<i64> {
    if state.is_organized() {
        return Some(0);
    }
    let key = state.digest();
    if let Some(&cost) = memo.get(&key) {
        return Some(cost);
    }

    // apply them and keep the cheapest
    // use a loop due to all the lifetimes
    let mut min_cost = None;
    for (fish, i, a, b, cost) in next_moves(state) {
        *state.tiles.get_mut(&a).unwrap() = Tile::Open;
        *state.tiles.get_mut(&b).unwrap() = Tile::Fish(fish);
        state.fish[fish.idx()][i] = b;
        if let Some(sub_cost) = min_energy(state, memo) {
            let total_cost = cost + sub_cost;
            min_cost = Some(min_cost.unwrap_or(std::i64::MAX).min(total_cost));
        }
        state.fish[fish.idx()][i] = a;
        *state.tiles.get_mut(&b).unwrap() = Tile::Open;
        *state.tiles.get_mut(&a).unwrap() = Tile::Fish(fish);
    }

    if let Some(min_cost) = min_cost {
        memo.insert(key, min_cost);
    }
    min_cost
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut state = parse(&text);
    println!("{:?}", min_energy(&mut state, &mut HashMap::new()).unwrap());
}
