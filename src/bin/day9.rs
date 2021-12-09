use std::collections::{BinaryHeap, HashSet};

type Grid = Vec<Vec<i32>>;
type Pt = (usize, usize);

fn is_low_point(grid: &Grid, row: usize, col: usize) -> bool {
    let val = grid[row][col];
    !(row > 0 && grid[row - 1][col] <= val
        || row < grid.len() - 1 && grid[row + 1][col] <= val
        || col > 0 && grid[row][col - 1] <= val
        || col < grid[0].len() - 1 && grid[row][col + 1] <= val)
}

fn part1(grid: &Grid) -> i32 {
    let risk = |i, j| if is_low_point(grid, i, j) { grid[i][j] + 1 } else { 0 };
    let row_risk = |i| (0..grid[0].len()).map(|j| risk(i, j)).sum::<i32>();
    (0..grid.len()).map(row_risk).sum()
}

fn explore(grid: &Grid, row: usize, col: usize, v: &mut HashSet<Pt>) {
    let val = grid[row][col];
    v.insert((row, col));
    for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if row == 0 && i < 0 || row == grid.len() - 1 && i > 0 {
            continue;
        }
        if col == 0 && j < 0 || col == grid[0].len() - 1 && j > 0 {
            continue;
        }
        let nbr_row = (row as i32 + i) as usize;
        let nbr_col = (col as i32 + j) as usize;
        let nbr_val = grid[nbr_row][nbr_col];
        if !v.contains(&(nbr_row, nbr_col)) && nbr_val != 9 && nbr_val > val {
            explore(grid, nbr_row as usize, nbr_col as usize, v);
        }
    }
}

fn part2(grid: &Grid) -> i32 {
    let mut sizes = BinaryHeap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 9 {
                let mut v = HashSet::new();
                explore(grid, row, col, &mut v);
                sizes.push(v.len() as i32);
            }
        }
    }
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

fn parse(s: &str) -> Grid {
    let atoi = |ch: u8| (ch - b'0') as i32;
    let parse_line = |s: &str| s.bytes().map(atoi).collect();
    s.lines().map(parse_line).collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let grid = parse(&text);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
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
        assert_eq!(15, part1(&parse(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1134, part2(&parse(INPUT)));
    }
}
