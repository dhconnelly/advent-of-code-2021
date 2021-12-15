use std::collections::{BinaryHeap, HashMap};

type Pt = (i32, i32);
type Graph = std::collections::HashMap<Pt, u8>;

#[derive(Eq, PartialEq)]
struct Node {
    pt: Pt,
    dist: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist).then_with(|| self.pt.cmp(&other.pt))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn nbrs((r, c): Pt) -> impl Iterator<Item = Pt> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(move |(dr, dc)| (r + dr, c + dc))
}

fn shortest_path(graph: &Graph, from: Pt, to: Pt) -> Option<i64> {
    let mut dists: HashMap<&Pt, i64> =
        graph.iter().map(|(pt, _)| (pt, std::i64::MAX)).collect();
    let mut q = BinaryHeap::new();
    q.push(Node { pt: from, dist: 0 });
    while let Some(Node { pt, dist }) = q.pop() {
        if pt == to {
            return Some(dist);
        }
        if dist > *dists.get(&pt).unwrap() {
            continue;
        }
        for nbr in nbrs(pt) {
            if let Some(nbr_val) = graph.get(&nbr) {
                let nbr_dist = dist + *nbr_val as i64;
                if nbr_dist < *dists.get(&nbr).unwrap() {
                    q.push(Node { pt: nbr, dist: nbr_dist });
                    *dists.get_mut(&nbr).unwrap() = nbr_dist;
                }
            }
        }
    }
    None
}

fn min_risk(graph: &Graph) -> i64 {
    let from = (0, 0);
    let to = (
        *graph.keys().map(|(row, _)| row).max().unwrap(),
        *graph.keys().map(|(_, col)| col).max().unwrap(),
    );
    shortest_path(&graph, from, to).unwrap()
}

fn inc_round(mut val: u8, i: u8, j: u8) -> u8 {
    val = (0..i).fold(val, |val, _| if val + 1 == 10 { 1 } else { val + 1 });
    (0..j).fold(val, |val, _| if val + 1 == 10 { 1 } else { val + 1 })
}

fn expand(graph: &Graph) -> Graph {
    let mut expanded = Graph::new();
    let height = graph.keys().map(|(r, _)| r).max().unwrap() + 1;
    let width = graph.keys().map(|(_, c)| c).max().unwrap() + 1;
    for i in 0..5 {
        for j in 0..5 {
            for (&(row, col), &val) in graph.iter() {
                let row = i * height + row;
                let col = j * width + col;
                expanded.insert((row, col), inc_round(val, i as u8, j as u8));
            }
        }
    }
    expanded
}

fn parse(s: &str) -> Graph {
    s.lines().enumerate().fold(Graph::new(), |acc, (row, line)| {
        line.trim().bytes().enumerate().fold(acc, |mut acc, (col, ch)| {
            acc.insert((row as i32, col as i32), ch - b'0');
            acc
        })
    })
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let graph = parse(&text);
    println!("{}", min_risk(&graph));
    println!("{}", min_risk(&expand(&graph)));
}
