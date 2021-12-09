type Grid = Vec<Vec<i32>>;

fn is_low_point(grid: &Grid, row: usize, col: usize) -> bool {
    let val = grid[row][col];
    if row > 0 && grid[row - 1][col] <= val {
        return false;
    }
    if row < grid.len() - 1 && grid[row + 1][col] <= val {
        return false;
    }
    if col > 0 && grid[row][col - 1] <= val {
        return false;
    }
    if col < grid[0].len() - 1 && grid[row][col + 1] <= val {
        return false;
    }
    true
}

fn part1(grid: &Grid) -> i32 {
    let mut risk = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_low_point(grid, row, col) {
                risk += grid[row][col] + 1;
            }
        }
    }
    risk
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
}
