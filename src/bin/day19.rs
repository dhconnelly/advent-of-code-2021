type Pt3 = (i64, i64, i64);
type Pts = std::collections::HashSet<Pt3>;

#[derive(Clone)]
struct Scan {
    beacons: Pts,
    scanners: Pts,
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
            let scanners = Pts::from([(0, 0, 0)]);
            Scan { beacons, scanners }
        })
        .collect()
}

fn maybe_align(scan1: &Scan, scan2: &Scan) -> Option<Scan> {
    for (x1, y1, z1) in &scan1.beacons {
        for (x2, y2, z2) in &scan2.beacons {
            let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
            let translate = |&(x, y, z)| (x + dx, y + dy, z + dz);
            let beacons2 = scan2.beacons.iter().map(translate).collect();
            let common = scan1.beacons.intersection(&beacons2).count();
            if common >= 12 {
                let beacons = beacons2.union(&scan1.beacons).copied().collect();
                let scanners2 = scan2.scanners.iter().map(translate).collect();
                let scanners =
                    scan1.scanners.union(&scanners2).copied().collect();
                return Some(Scan { scanners, beacons });
            }
        }
    }
    None
}

fn rotate(scan: &Scan, rot: impl Fn(&Pt3) -> Pt3 + Copy) -> Scan {
    let beacons = scan.beacons.iter().map(rot).collect();
    let scanners = scan.scanners.iter().map(rot).collect();
    Scan { beacons, scanners }
}

fn rotate_z(scan: &Scan) -> Scan {
    rotate(scan, |&(x, y, z)| (-y, x, z))
}

fn rotate_x(scan: &Scan) -> Scan {
    rotate(scan, |&(x, y, z)| (x, -z, y))
}

fn rotate_y(scan: &Scan) -> Scan {
    rotate(scan, |&(x, y, z)| (-z, y, x))
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

fn align(mut scans: Vec<Scan>) -> Scan {
    let mut done = vec![false; scans.len()];
    for i in 0..scans.len() {
        for j in 0..scans.len() {
            if i == j || done[i] || done[j] {
                continue;
            }
            for scan2 in rotations(&scans[j]) {
                if let Some(aligned) = maybe_align(&scans[i], &scan2) {
                    scans[i] = aligned;
                    done[j] = true;
                    break;
                }
            }
        }
    }
    let idx = done.iter().position(|done| !done).unwrap();
    scans.remove(idx)
}

fn manhattan(&(x1, y1, z1): &Pt3, &(x2, y2, z2): &Pt3) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

fn max_dist(pts: &Vec<&Pt3>) -> i64 {
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
    println!("{}", max_dist(&scan.scanners.iter().collect()));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_part1() {
        let scans = parse(INPUT);
        let scan = align(scans);
        assert_eq!(79, scan.beacons.len());
        assert_eq!(3621, max_dist(&scan.scanners.iter().collect()));
    }
}
