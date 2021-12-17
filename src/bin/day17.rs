#[derive(Debug, Clone, Copy)]
struct Pt(i32, i32);

const MAX_Y_LOL: i32 = 1000;

impl Pt {
    fn add(&mut self, slope: &Pt) {
        self.0 += slope.0;
        self.1 += slope.1;
    }

    fn drag(&mut self) {
        // assume x > 0 always
        if self.0 > 0 {
            self.0 -= 1;
        }
        self.1 -= 1;
    }
}

#[derive(Debug)]
struct Rect(Pt, Pt);

impl Rect {
    fn contains(&self, pt: &Pt) -> bool {
        self.0 .0 <= pt.0
            && pt.0 <= self.1 .0
            && self.0 .1 <= pt.1
            && pt.1 <= self.1 .1
    }
}

fn cannot_reach(target: &Rect, pos: &Pt) -> bool {
    pos.0 > target.1 .0 || pos.1 < target.0 .1
}

fn max_y_reaching(target: &Rect, mut slope: Pt) -> Option<i32> {
    let mut pos = Pt(0, 0);
    let mut max_y = 0;
    while !cannot_reach(target, &pos) {
        pos.add(&slope);
        slope.drag();
        max_y = max_y.max(pos.1);
        if target.contains(&pos) {
            return Some(max_y);
        }
    }
    None
}

fn part1(target: &Rect) -> i32 {
    let mut max_y = std::i32::MIN;
    for dy in target.0 .1..MAX_Y_LOL {
        for dx in 1..=target.1 .0 {
            let slope = Pt(dx, dy);
            if let Some(y) = max_y_reaching(target, slope) {
                max_y = max_y.max(y);
            }
        }
    }
    max_y
}

fn part2(target: &Rect) -> i32 {
    let mut n = 0;
    for dy in target.0 .1..MAX_Y_LOL {
        for dx in 1..=target.1 .0 {
            let slope = Pt(dx, dy);
            if let Some(_) = max_y_reaching(target, slope) {
                n += 1;
            }
        }
    }
    n
}

fn parse(s: &str) -> Rect {
    let s = s.trim().split_once(": ").unwrap().1;
    let (x, y) = s.split_once(", ").unwrap();
    let (x, y) = (x.split_once('=').unwrap().1, y.split_once('=').unwrap().1);
    let (x0, x1) = x.split_once("..").unwrap();
    let (x0, x1) = (x0.parse().unwrap(), x1.parse().unwrap());
    let (y0, y1) = y.split_once("..").unwrap();
    let (y0, y1) = (y0.parse().unwrap(), y1.parse().unwrap());
    Rect(Pt(x0, y0), Pt(x1, y1))
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let target = parse(&text);
    println!("{}", part1(&target));
    println!("{}", part2(&target));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test() {
        assert_eq!(45, part1(&parse(&INPUT)));
        assert_eq!(112, part2(&parse(&INPUT)));
    }
}
