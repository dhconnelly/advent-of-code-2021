use std::collections::HashSet;
use std::fmt;

type Pt = (i32, i32);

struct Grid {
    width: i32,
    height: i32,
    dots: HashSet<Pt>,
}

impl Grid {
    fn flip(&mut self, pt: Pt, fold: Pt) {
        let flipped = if fold.0 > 0 {
            let d = pt.0 - fold.0;
            (fold.0 - d, pt.1)
        } else {
            let d = pt.1 - fold.1;
            (pt.0, fold.1 - d)
        };
        self.dots.insert(flipped);
        self.dots.remove(&pt);
    }

    fn flip_all(&mut self, pts: &[Pt], fold: Pt) {
        for pt in pts {
            self.flip(*pt, fold);
        }
        self.height = if fold.1 > 0 { fold.1 } else { self.height };
        self.width = if fold.0 > 0 { fold.0 } else { self.width };
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let z = if self.dots.contains(&(x, y)) { '#' } else { '.' };
                write!(f, "{}", z)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn apply(grid: &mut Grid, fold: Pt) {
    let to_flip: Vec<Pt> = grid
        .dots
        .iter()
        .filter(|&&(x, y)| fold.0 > 0 && x > fold.0 || fold.1 > 0 && y > fold.1)
        .copied()
        .collect();
    grid.flip_all(&to_flip, fold);
}

fn parse(s: &str) -> (Grid, Vec<Pt>) {
    let (dots, folds) = s.split_once("\n\n").unwrap();
    let dots: HashSet<Pt> = dots
        .lines()
        .map(|line| line.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let width = dots.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = dots.iter().map(|(_, y)| y).max().unwrap() + 1;
    let folds = folds
        .lines()
        .map(|line| line.trim()[11..].split_once('=').unwrap())
        .map(|(coord, val)| (coord, val.parse().unwrap()))
        .map(|(coord, val)| if coord == "x" { (val, 0) } else { (0, val) })
        .collect();
    (Grid { dots, width, height }, folds)
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (mut grid, folds) = parse(&text);
    apply(&mut grid, folds[0]);
    println!("{}", grid.dots.len());
    folds[1..].iter().for_each(|&fold| apply(&mut grid, fold));
    println!("{}", grid);
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "6,10
                                  0,14
                                  9,10
                                  0,3
                                  10,4
                                  4,11
                                  6,0
                                  6,12
                                  4,1
                                  0,13
                                  10,12
                                  3,4
                                  3,0
                                  8,4
                                  1,10
                                  2,14
                                  8,10
                                  9,0

                                  fold along y=7
                                  fold along x=5";

    #[test]
    fn test_part1() {
        let (mut grid, folds) = parse(INPUT);
        apply(&mut grid, folds[0]);
        assert_eq!(17, grid.dots.len());
    }
}
