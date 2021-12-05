type Pt = (i32, i32);
type Line = (Pt, Pt);
type Grid = std::collections::HashMap<Pt, i32>;

fn atoi(s: &str) -> i32 {
    s.parse().unwrap()
}

fn parse_line(s: &str) -> Line {
    let (from, to) = s.split_once(" -> ").unwrap();
    let (from_x, from_y) = from.split_once(',').unwrap();
    let (to_x, to_y) = to.split_once(',').unwrap();
    ((atoi(from_x), atoi(from_y)), (atoi(to_x), atoi(to_y)))
}

fn is_horiz(l: &Line) -> bool {
    l.0 .1 == l.1 .1
}

fn is_vert(l: &Line) -> bool {
    l.0 .0 == l.1 .0
}

fn slope(l: &Line) -> Pt {
    if is_horiz(l) {
        if l.1 .0 - l.0 .0 > 0 {
            (1, 0)
        } else {
            (-1, 0)
        }
    } else {
        if l.1 .1 - l.0 .1 > 0 {
            (0, 1)
        } else {
            (0, -1)
        }
    }
}

fn apply_line(l: &Line, g: &mut Grid) {
    assert!(is_horiz(l) || is_vert(l));
    let d = slope(l);
    let mut pt = l.0;
    while pt != l.1 {
        *g.entry(pt).or_default() += 1;
        pt = (pt.0 + d.0, pt.1 + d.1);
    }
    *g.entry(pt).or_default() += 1;
}

fn part1(lines: &[Line]) -> usize {
    let mut g = Grid::new();
    for line in lines.iter().filter(|l| is_horiz(l) || is_vert(l)) {
        apply_line(line, &mut g);
    }
    g.values().filter(|count| **count > 1).count()
}

fn parse_lines(s: &str) -> Vec<Line> {
    s.lines().map(parse_line).collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let lines = parse_lines(&text);
    println!("{:?}", part1(&lines));
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
        assert_eq!(5, part1(&lines));
    }
}
