use std::collections::HashSet;

type Pt3 = (i64, i64, i64);
type Grid = HashSet<Pt3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Command {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Step {
    cmd: Command,
    lo: Pt3,
    hi: Pt3,
}

impl Step {
    fn cube(&self) -> Cube {
        Cube { lo: self.lo, hi: self.hi }
    }
}

fn apply(
    &Step { cmd, mut lo, mut hi }: &Step,
    grid: &mut Grid,
    bounds: Option<(i64, i64)>,
) {
    if let Some((min, max)) = bounds {
        lo = (lo.0.max(min), lo.1.max(min), lo.2.max(min));
        hi = (hi.0.min(max), hi.1.min(max), hi.2.min(max));
    }
    for x in lo.0..=hi.0 {
        for y in lo.1..=hi.1 {
            for z in lo.2..=hi.2 {
                match cmd {
                    Command::On => grid.insert((x, y, z)),
                    Command::Off => grid.remove(&(x, y, z)),
                };
            }
        }
    }
}

fn parse_step(s: &str) -> Step {
    let parse_coord_range = |s: &str| {
        let s = s.split_once('=').unwrap().1;
        let (lo, hi) = s.split_once("..").unwrap();
        (lo.parse().unwrap(), hi.parse().unwrap())
    };
    let (cmd, range) = s.split_once(' ').unwrap();
    let coords: Vec<(i64, i64)> =
        range.split(',').map(parse_coord_range).collect();
    Step {
        cmd: if cmd == "on" { Command::On } else { Command::Off },
        lo: (coords[0].0, coords[1].0, coords[2].0),
        hi: (coords[0].1, coords[1].1, coords[2].1),
    }
}

fn parse(s: &str) -> Vec<Step> {
    s.lines().map(parse_step).collect()
}

fn apply_all<'a>(
    steps: impl Iterator<Item = &'a Step>,
    grid: &mut Grid,
    bounds: Option<(i64, i64)>,
) {
    steps.for_each(|step| apply(step, grid, bounds));
}

fn part1(steps: &[Step]) -> i64 {
    let mut grid = Grid::new();
    apply_all(steps.iter(), &mut grid, Some((-50, 50)));
    grid.len() as i64
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube {
    lo: Pt3,
    hi: Pt3,
}

impl Cube {
    fn volume(&self) -> i64 {
        (self.hi.0 - self.lo.0 + 1)
            * (self.hi.1 - self.lo.1 + 1)
            * (self.hi.2 - self.lo.2 + 1)
    }

    fn contains(&self, other: &Cube) -> bool {
        self.lo.0 <= other.lo.0
            && other.hi.0 <= self.hi.0
            && self.lo.1 <= other.lo.1
            && other.hi.1 <= self.hi.1
            && self.lo.2 <= other.lo.2
            && other.hi.2 <= self.hi.2
    }

    fn exclusive(&self, other: &Cube) -> bool {
        let (lo1, hi1) = (self.lo, self.hi);
        let (lo2, hi2) = (other.lo, other.hi);
        hi1.0 < lo2.0
            || lo1.0 > hi2.0
            || hi1.1 < lo2.1
            || lo1.1 > hi2.1
            || hi2.2 < lo2.2
            || lo1.2 > hi2.2
    }

    fn remove_interior(&self, interior: &Cube) -> Vec<Cube> {
        vec![
            Cube {
                lo: (self.lo.0, self.lo.1, self.lo.2),
                hi: (self.hi.0, self.hi.1, interior.lo.2),
            }, // bottom
            Cube {
                lo: (self.lo.0, self.lo.1, interior.hi.2),
                hi: (self.hi.0, self.hi.1, self.hi.2),
            }, // top
            // on level of interior (E):
            // aaaaa
            // b E c
            // ddddd
            Cube {
                lo: (self.lo.0, interior.hi.1, interior.lo.2),
                hi: (self.hi.0, self.hi.1, interior.hi.2),
            }, // a
            Cube {
                lo: (self.lo.0, interior.lo.1, interior.lo.2),
                hi: (interior.lo.0, interior.hi.1, interior.hi.2),
            }, // b
            Cube {
                lo: (interior.hi.0, interior.lo.1, interior.lo.2),
                hi: (self.hi.0, interior.hi.1, interior.hi.2),
            }, // c
            Cube {
                lo: (self.lo.0, self.lo.1, interior.lo.2),
                hi: (self.hi.0, interior.lo.1, interior.hi.2),
            }, // d
        ]
    }

    fn remove_overlap(&self, other: &Cube) -> Vec<Cube> {
        let overlap_lo = (
            self.lo.0.max(other.lo.0),
            self.lo.1.max(other.lo.1),
            self.lo.2.max(other.lo.2),
        );
        let overlap_hi = (
            self.hi.0.min(other.hi.0),
            self.hi.1.min(other.hi.1),
            self.hi.2.min(other.hi.2),
        );
        let overlap = Cube { lo: overlap_lo, hi: overlap_hi };
        println!("overlap of {:?} and {:?} = {:?}", self, other, overlap);
        // now how to remove it?
        Vec::new()
    }
}

// shout out to https://stackoverflow.com/a/12771129/12903251
fn part2(steps: &[Step]) -> i64 {
    let mut cubes = HashSet::from([steps[0].cube()]);
    let mut q: Vec<Step> = steps.iter().cloned().collect();
    'outer: while let Some(step) = q.pop() {
        let cube = step.cube();
        let mut add = HashSet::new();
        let mut remove = HashSet::new();
        for prev in &cubes {
            if cube.exclusive(prev) {
                continue;
            } else if prev.contains(&cube) {
                continue 'outer;
            } else if cube.contains(prev) {
                let exterior = cube.remove_interior(prev);
                q.extend(exterior.into_iter().map(|cube| Step {
                    cmd: step.cmd,
                    lo: cube.lo,
                    hi: cube.hi,
                }));
                if step.cmd == Command::Off {
                    remove.insert(prev.clone());
                }
            } else {
                let prev_rest = prev.remove_overlap(&cube);
                let cube_rest = cube.remove_overlap(prev);
                if step.cmd == Command::Off {
                    remove.insert(prev.clone());
                    add.extend(prev_rest.into_iter());
                }
                q.extend(cube_rest.into_iter().map(|cube| Step {
                    cmd: step.cmd,
                    lo: cube.lo,
                    hi: cube.hi,
                }));
            }
        }
        cubes.extend(add.into_iter());
        cubes.retain(|cube| !remove.contains(cube));
    }
    println!("{:?}", cubes.iter().next());
    cubes.iter().map(Cube::volume).sum()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let steps = parse(&text);
    println!("{}", part1(&steps));
    println!("{}", part2(&steps));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    static INPUT2: &'static str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    static INPUT3: &'static str = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn test_part1() {
        let steps = parse(INPUT);
        let mut grid = Grid::new();
        apply_all(steps.iter(), &mut grid, Some((-50, 50)));
        assert_eq!(39, grid.len());

        let steps = parse(INPUT2);
        let mut grid = Grid::new();
        apply_all(steps.iter(), &mut grid, Some((-50, 50)));
        assert_eq!(590784, grid.len());
    }

    #[test]
    fn test_part2() {
        let steps = parse(INPUT3);
        assert_eq!(2758514936282235, part2(&steps));
    }
}
