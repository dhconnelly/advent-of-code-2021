use std::collections::{BinaryHeap, HashMap, HashSet};

type Pt = (i32, i32);
type Grid = HashMap<Pt, i32>;

fn is_low_point(g: &Grid, &(i, j): &Pt) -> bool {
    let val = g.get(&(i, j)).unwrap();
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
        .into_iter()
        .all(|nbr| !g.contains_key(&nbr) || g.get(&nbr).unwrap() > val)
}

fn low_points(g: &Grid) -> Vec<&Pt> {
    g.keys().filter(|pt| is_low_point(&g, pt)).collect()
}

fn part1(g: &Grid, pts: &[&Pt]) -> i32 {
    pts.iter().map(|pt| g.get(pt).unwrap() + 1).sum()
}

fn explore(g: &Grid, pt @ &(i, j): &Pt, v: &mut HashSet<Pt>) -> i32 {
    let val = *g.get(pt).unwrap();
    v.insert(*pt);
    for nbr in &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
        if g.contains_key(nbr) {
            let nbr_val = *g.get(nbr).unwrap();
            if !v.contains(nbr) && nbr_val != 9 && nbr_val > val {
                explore(g, nbr, v);
            }
        }
    }
    v.len() as i32
}

fn part2(g: &Grid, low_points: &[&Pt]) -> i32 {
    let size = |&pt| explore(g, pt, &mut HashSet::new());
    let mut sizes = low_points.iter().map(size).collect::<BinaryHeap<i32>>();
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

fn parse(s: &str) -> Grid {
    let mut g = Grid::new();
    for (row, line) in s.lines().enumerate() {
        for (col, val) in line.bytes().enumerate() {
            g.insert((row as i32, col as i32), (val - b'0') as i32);
        }
    }
    g
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = parse(&text);
    let low_points = low_points(&grid);
    println!("{}", part1(&grid, &low_points));
    println!("{}", part2(&grid, &low_points));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = r"2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_part1() {
        let g = parse(INPUT);
        assert_eq!(15, part1(&g, &low_points(&g)));
    }

    #[test]
    fn test_part2() {
        let g = parse(INPUT);
        assert_eq!(1134, part2(&g, &low_points(&g)));
    }
}
