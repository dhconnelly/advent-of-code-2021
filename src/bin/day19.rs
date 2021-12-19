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

fn find(scan1: &Scan, scan2: &Scan) -> Option<Vec<Pt3>> {
    println!("{:?}", scan1);
    println!("{:?}", scan2);
    None
}

fn rotate_z(scan: &Scan) -> Scan {
    scan.iter().map(|&(x, y, z)| (-y, x, z)).collect()
}

fn rotate_x(scan: &Scan) -> Scan {
    scan.iter().map(|&(x, y, z)| (x, -z, y)).collect()
}

fn rotate_y(scan: &Scan) -> Scan {
    scan.iter().map(|&(x, y, z)| (-z, y, x)).collect()
}

fn rotations(scan: &Scan) -> Vec<Scan> {
    let mut scan = scan.clone();
    let mut rotations = Vec::new();
    for _ in 0..4 {
        for _ in 0..4 {
            scan = rotate_z(&scan);
            rotations.push(scan.clone());
        }
        for _ in 0..2 {
            scan = rotate_x(&scan);
            rotations.push(scan.clone());
            scan = rotate_x(&scan);
        }
        scan = rotate_y(&scan);
    }
    rotations
}

fn find_common_beacons(scan1: &Scan, scan2: &Scan) -> Option<Vec<Pt3>> {
    for scan2 in rotations(scan2) {
        if let Some(beacons) = find(&scan1, &scan2) {
            return Some(beacons);
        }
    }
    None
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let scans = parse(&text);
    println!("{:?}", scans);
    for scan in rotations(&scans[0]) {
        println!("{:?}", scan);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "";

    #[test]
    fn test_part1() {}
}
