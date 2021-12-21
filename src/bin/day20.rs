use std::collections::{HashMap, HashSet};

type Pt = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bit {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct BitMap {
    default: Bit,
    bits: HashMap<Pt, Bit>,
}

fn parse(s: &str) -> (Vec<Bit>, BitMap) {
    let ctob = |ch| if ch == '.' { Bit::Off } else { Bit::On };
    let (fst, snd) = s.split_once("\n\n").unwrap();
    let alg: Vec<Bit> = fst.trim().chars().map(ctob).collect();
    let lines = snd.trim().lines().enumerate();
    let bits = lines
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(move |(c, ch)| ((r as i32, c as i32), ctob(ch)))
        })
        .collect();
    let map = BitMap { default: Bit::Off, bits };
    (alg, map)
}

static NBRS: &'static [Pt] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn nbrs_of((r, c): Pt) -> impl Iterator<Item = Pt> {
    NBRS.iter().map(move |(dr, dc)| (r + dr, c + dc))
}

fn bits_to_idx(bits: impl Iterator<Item = Bit>) -> usize {
    bits.fold(0, |acc, bit| match bit {
        Bit::Off => acc << 1,
        Bit::On => (acc << 1) | 1,
    })
}

fn next_default(alg: &[Bit], map: &BitMap) -> Bit {
    let bits = (0..9).map(|_| &map.default).copied();
    let idx = bits_to_idx(bits);
    alg[idx]
}

fn get_idx(map: &BitMap, pt: Pt) -> usize {
    let bits = nbrs_of(pt).map(|pt| map.bits.get(&pt).unwrap_or(&map.default));
    bits_to_idx(bits.copied())
}

fn apply(alg: &[Bit], map: &BitMap) -> BitMap {
    let mut edge = HashSet::new();
    let mut next =
        BitMap { default: next_default(alg, map), bits: HashMap::new() };
    for &pt in map.bits.keys() {
        edge.extend(nbrs_of(pt));
        let idx = get_idx(map, pt);
        next.bits.insert(pt, alg[idx]);
    }
    for pt in edge {
        if next.bits.contains_key(&pt) {
            continue;
        }
        let idx = get_idx(map, pt);
        next.bits.insert(pt, alg[idx]);
    }
    next
}

fn apply_n(alg: &[Bit], img: BitMap, n: usize) -> BitMap {
    (0..n).fold(img, |acc, _| apply(alg, &acc))
}

fn on_bits(img: &BitMap) -> usize {
    img.bits.values().filter(|&&bit| bit == Bit::On).count()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (alg, img) = parse(&text);
    println!("{}", on_bits(&apply_n(&alg, img.clone(), 2)));
    println!("{}", on_bits(&apply_n(&alg, img.clone(), 50)));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part1() {
        let (alg, img) = parse(INPUT);
        let img = apply(&alg, &img);
        let img = apply(&alg, &img);
        assert_eq!(
            35,
            img.bits.values().filter(|&&bit| bit == Bit::On).count()
        );
    }
}
