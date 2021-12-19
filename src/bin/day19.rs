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

fn maybe_align(scan1: &Scan, scan2: &Scan) -> Option<Scan> {
    for (x1, y1, z1) in scan1 {
        for (x2, y2, z2) in scan2 {
            let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
            let scan2_shifted = scan2
                .iter()
                .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
                .collect();
            let common: Vec<_> =
                scan1.intersection(&scan2_shifted).copied().collect();
            if common.len() >= 12 {
                return Some(scan2_shifted.union(scan1).copied().collect());
            }
        }
    }
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

fn align(scans: &mut [Scan]) {
    for i in 0..scans.len() {
        for j in 0..scans.len() {
            if i == j || scans[i].is_empty() || scans[j].is_empty() {
                continue;
            }
            for scan2 in rotations(&scans[j]) {
                if let Some(aligned) = maybe_align(&scans[i], &scan2) {
                    scans[i] = aligned;
                    scans[j] = Scan::new();
                    break;
                }
            }
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let mut scans = parse(&text);
    align(&mut scans);
    let beacons: Scan =
        scans.iter().flat_map(|scan| scan.iter().copied()).collect();
    println!("{}", beacons.len());
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "";

    #[test]
    fn test_part1() {}
}
