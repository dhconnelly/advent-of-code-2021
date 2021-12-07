fn parse(s: &str) -> Vec<i32> {
    s.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn min_dist_sum(xs: &[i32], dist: impl Fn(i32, i32) -> i32) -> i32 {
    let min = *xs.iter().min().unwrap();
    let max = *xs.iter().max().unwrap();
    let dist_sum = |to: i32| xs.iter().map(|&x| dist(x, to)).sum();
    (min..=max).map(dist_sum).min().unwrap()
}

fn part1(xs: &[i32]) -> i32 {
    min_dist_sum(xs, |a, b| (a - b).abs())
}

fn part2(xs: &[i32]) -> i32 {
    min_dist_sum(xs, |a, b| {
        let n = (a - b).abs();
        (n * (n + 1)) / 2
    })
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let xs = parse(&text);
    println!("{}", part1(&xs));
    println!("{}", part2(&xs));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let xs: Vec<i32> = parse(INPUT);
        assert_eq!(37, part1(&xs));
    }

    #[test]
    fn test_part2() {
        let xs: Vec<i32> = parse(INPUT);
        assert_eq!(168, part2(&xs));
    }
}
