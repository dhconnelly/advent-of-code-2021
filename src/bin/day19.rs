use std::collections::HashSet;

type Pt3 = (i64, i64, i64);

#[derive(Clone)]
struct Scan {
    beacons: HashSet<Pt3>,
    scanners: HashSet<Pt3>,
}

fn parse(s: &str) -> Vec<Scan> {
    let parse_pt = |s: &str| {
        let mut coords = s.split(',');
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
    };
    s.split("\n\n")
        .map(|s| {
            let beacons = s.lines().skip(1).map(parse_pt).collect();
            let scanners = HashSet::from([(0, 0, 0)]);
            Scan { beacons, scanners }
        })
        .collect()
}

fn maybe_align(scan1: &Scan, scan2: &Scan) -> Option<Scan> {
    for (x1, y1, z1) in &scan1.beacons {
        for (x2, y2, z2) in &scan2.beacons {
            let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
            let beacons2_shifted = scan2
                .beacons
                .iter()
                .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
                .collect();
            let common: Vec<_> = scan1
                .beacons
                .intersection(&beacons2_shifted)
                .copied()
                .collect();
            if common.len() >= 12 {
                let beacons =
                    beacons2_shifted.union(&scan1.beacons).copied().collect();
                let scanners2_shifted: HashSet<Pt3> = scan2
                    .scanners
                    .iter()
                    .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
                    .collect();
                let scanners =
                    scanners2_shifted.union(&scan1.scanners).copied().collect();
                return Some(Scan { scanners, beacons });
            }
        }
    }
    None
}

fn rotate(scan: &mut Scan, rot: impl Fn(&Pt3) -> Pt3 + Copy) {
    scan.beacons = scan.beacons.iter().map(rot).collect();
    scan.scanners = scan.scanners.iter().map(rot).collect();
}

fn rotate_z(scan: &mut Scan) {
    rotate(scan, |&(x, y, z)| (-y, x, z))
}

fn rotate_x(scan: &mut Scan) {
    rotate(scan, |&(x, y, z)| (x, -z, y))
}

fn rotate_y(scan: &mut Scan) {
    rotate(scan, |&(x, y, z)| (-z, y, x))
}

fn align(mut scans: Vec<Scan>) -> Scan {
    let mut done = vec![false; scans.len()];
    for i in 0..scans.len() {
        for j in 0..scans.len() {
            if i == j || done[i] || done[j] {
                continue;
            }
            for _ in 0..4 {
                for _ in 0..4 {
                    rotate_z(&mut scans[j]);
                    if let Some(aligned) = maybe_align(&scans[i], &scans[j]) {
                        scans[i] = aligned;
                        done[j] = true;
                        break;
                    }
                }
                if done[j] {
                    break;
                }
                for _ in 0..2 {
                    rotate_x(&mut scans[j]);
                    if let Some(aligned) = maybe_align(&scans[i], &scans[j]) {
                        scans[i] = aligned;
                        done[j] = true;
                        break;
                    }
                    rotate_x(&mut scans[j]);
                }
                if done[j] {
                    break;
                }
                rotate_y(&mut scans[j]);
            }
        }
    }
    let idx = done.iter().position(|&done| !done).unwrap();
    scans.remove(idx)
}

fn manhattan(&(x1, y1, z1): &Pt3, &(x2, y2, z2): &Pt3) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

fn max_dist(pts: &Vec<Pt3>) -> i64 {
    let mut max = std::i64::MIN;
    for pt1 in pts {
        for pt2 in pts {
            if pt1 != pt2 {
                max = max.max(manhattan(pt1, pt2));
            }
        }
    }
    max
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let scans = parse(&text);
    let scan = align(scans);
    println!("{}", scan.beacons.len());
    println!("{}", max_dist(&scan.scanners.iter().cloned().collect()));
}
