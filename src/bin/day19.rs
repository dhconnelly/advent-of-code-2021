type Pt3 = (i64, i64, i64);
type Scan = std::collections::HashSet<Pt3>;

fn parse(s: &str) -> Vec<Scan> {
    let parse_pt = |s: &str| {
        let mut coords = s.split(',');
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
    };
    s.split("\n\n").map(|s| s.lines().skip(1).map(parse_pt).collect()).collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let scans = parse(&text);
    println!("{:?}", scans);
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "";

    #[test]
    fn test_part1() {}
}
