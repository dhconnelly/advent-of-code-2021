type Pt = (i32, i32);
type Line = (Pt, Pt);
type Grid = std::collections::HashMap<Pt, i32>;

fn slope(l: &Line) -> Pt {
    let dx = (l.1 .0 - l.0 .0).signum();
    let dy = (l.1 .1 - l.0 .1).signum();
    (dx, dy)
}

fn steps(l: &Line) -> i32 {
    (l.1 .0 - l.0 .0).abs().max((l.1 .1 - l.0 .1).abs())
}

fn apply_line(g: Grid, l: &Line) -> Grid {
    let d = slope(l);
    (0..=steps(l)).fold(g, |mut g, i| {
        let pt = (l.0 .0 + i * d.0, l.0 .1 + i * d.1);
        *g.entry(pt).or_default() += 1;
        g
    })
}

fn not_diag(l: &&Line) -> bool {
    l.0 .1 == l.1 .1 || l.0 .0 == l.1 .0
}

fn intersections(lines: &[Line], filter: impl Fn(&&Line) -> bool) -> usize {
    let grid = lines.iter().filter(filter).fold(Grid::new(), apply_line);
    grid.values().filter(|&&count| count > 1).count()
}

fn atoi(s: &str) -> i32 {
    s.parse().unwrap()
}

fn parse_line(s: &str) -> Line {
    let (from, to) = s.split_once(" -> ").unwrap();
    let (from_x, from_y) = from.split_once(',').unwrap();
    let (to_x, to_y) = to.split_once(',').unwrap();
    ((atoi(from_x), atoi(from_y)), (atoi(to_x), atoi(to_y)))
}

fn parse_lines(s: &str) -> Vec<Line> {
    s.lines().map(parse_line).collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let lines = parse_lines(&text);
    println!("{:?}", intersections(&lines, not_diag));
    println!("{:?}", intersections(&lines, |_| true));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let lines = parse_lines(INPUT);
        assert_eq!(5, intersections(&lines, not_diag));
    }

    #[test]
    fn test_part2() {
        let lines = parse_lines(INPUT);
        assert_eq!(12, intersections(&lines, |_| true));
    }
}
