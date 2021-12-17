type Pt = (i32, i32);
type Grid = std::collections::HashMap<Pt, u8>;

fn nbrs(pt @ (row, col): Pt) -> impl Iterator<Item = Pt> {
    let row_nbrs = move |dr| (-1..=1).map(move |dc| (row + dr, col + dc));
    (-1..=1).flat_map(row_nbrs).filter(move |nbr| nbr != &pt)
}

fn step(grid: &mut Grid) -> usize {
    let mut steps = 0;
    let mut q = Vec::from_iter(grid.keys().copied());
    while let Some(pt) = q.pop() {
        if let Some(val) = grid.get_mut(&pt) {
            *val += 1;
            if *val == 10 {
                q.extend(nbrs(pt));
                steps += 1;
            }
        }
    }
    grid.values_mut().filter(|val| **val > 9).for_each(|val| *val = 0);
    steps
}

fn part1(mut grid: Grid) -> usize {
    (0..100).map(|_| step(&mut grid)).sum()
}

fn part2(mut grid: Grid) -> usize {
    (1..).find(|_| step(&mut grid) == grid.len()).unwrap()
}

fn parse(s: &str) -> Grid {
    s.lines().enumerate().fold(Grid::new(), |grid, (row, line)| {
        line.bytes().enumerate().fold(grid, |mut grid, (col, ch)| {
            grid.insert((row as i32, col as i32), ch - b'0');
            grid
        })
    })
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = parse(&text);
    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(1656, part1(parse(&INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(195, part2(parse(&INPUT)));
    }
}
