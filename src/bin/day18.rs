use std::fmt;
use std::str;

#[derive(Debug, Clone)]
enum Num {
    Regular(u64),
    Pair(Box<Num>, Box<Num>),
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Regular(num) => write!(f, "{}", num)?,
            Num::Pair(lhs, rhs) => write!(f, "[{}, {}]", lhs, rhs)?,
        }
        Ok(())
    }
}

fn add_to_leftmost(num: &mut Num, value: u64) {
    match num {
        Num::Regular(num) => *num += value,
        Num::Pair(lhs, _) => add_to_leftmost(lhs.as_mut(), value),
    }
}

fn add_to_rightmost(num: &mut Num, value: u64) {
    match num {
        Num::Regular(num) => *num += value,
        Num::Pair(_, rhs) => add_to_rightmost(rhs.as_mut(), value),
    }
}

fn explode(num: &mut Num, level: usize) -> (Option<u64>, Option<u64>, bool) {
    match num {
        Num::Regular(_) => (None, None, false),

        Num::Pair(lhs, rhs) if level == 4 => {
            let (lhs, rhs) = (lhs.as_ref(), rhs.as_ref());
            if let (Num::Regular(x), Num::Regular(y)) = (lhs, rhs) {
                let (l, r) = (Some(*x), Some(*y));
                *num = Num::Regular(0);
                return (l, r, true);
            }
            panic!("invalid exploding pair");
        }

        Num::Pair(ref mut lhs, ref mut rhs) => {
            let (l, r, lhs_exploded) = explode(lhs, level + 1);
            if let Some(r) = r {
                add_to_leftmost(rhs, r);
            }
            if lhs_exploded {
                return (l, None, true);
            }
            let (l, r, rhs_exploded) = explode(rhs, level + 1);
            if let Some(l) = l {
                add_to_rightmost(lhs, l);
            }
            (None, r, rhs_exploded)
        }
    }
}

fn split(num: &mut Num) -> bool {
    match num {
        Num::Regular(value) if *value >= 10 => {
            *num = Num::Pair(
                Box::new(Num::Regular(*value / 2)),
                Box::new(Num::Regular(*value - (*value / 2))),
            );
            true
        }
        Num::Regular(_) => false,
        Num::Pair(lhs, rhs) => split(lhs.as_mut()) || split(rhs.as_mut()),
    }
}

fn reduce(num: &mut Num) {
    let mut reducing = true;
    while reducing {
        reducing = explode(num, 0).2 || split(num);
    }
}

fn add(lhs: Num, rhs: Num) -> Num {
    let mut sum = Num::Pair(Box::new(lhs), Box::new(rhs));
    reduce(&mut sum);
    sum
}

fn sum(nums: &[Num]) -> Num {
    nums.iter().skip(1).cloned().fold(nums[0].clone(), add)
}

fn magnitude(num: Num) -> u64 {
    match num {
        Num::Regular(value) => value,
        Num::Pair(lhs, rhs) => 3 * magnitude(*lhs) + 2 * magnitude(*rhs),
    }
}

fn parse(chars: &mut impl Iterator<Item = char>) -> Num {
    match chars.next() {
        None => panic!("unexpected eof"),
        Some('[') => {
            let lhs = parse(chars);
            assert_eq!(Some(','), chars.next());
            let rhs = parse(chars);
            assert_eq!(Some(']'), chars.next());
            Num::Pair(Box::new(lhs), Box::new(rhs))
        }
        Some(ch) => Num::Regular(ch.to_digit(10).unwrap() as u64),
    }
}

fn parse_num(s: &str) -> Num {
    parse(&mut s.chars())
}

fn parse_nums(s: &str) -> Vec<Num> {
    s.lines().map(str::trim).map(parse_num).collect()
}

fn part2(nums: &[Num]) -> u64 {
    let mut max = std::u64::MIN;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j {
                let m = magnitude(add(nums[i].clone(), nums[j].clone()));
                if m > max {
                    max = m;
                }
            }
        }
    }
    max
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let nums = parse_nums(&text);
    println!("{}", magnitude(sum(&nums)));
    println!("{}", part2(&nums));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str =
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_sum() {
        let nums = parse_nums(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
             [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
             [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
             [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
             [7,[5,[[3,8],[1,4]]]]
             [[2,[2,2]],[8,[8,1]]]
             [2,9]
             [1,[[[9,3],9],[[9,0],[0,7]]]]
             [[[5,[7,4]],7],1]
             [[[[4,2],2],6],[8,7]]",
        );
        let expected =
            parse_num("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(expected.to_string(), sum(&nums).to_string());
    }

    #[test]
    fn test_explode() {
        for (before, after, pair) in &[
            (
                "[[[[[9,8],1],2],3],4]",
                "[[[[0,9],2],3],4]",
                (Some(9), None),
            ),
            (
                "[7,[6,[5,[4,[3,2]]]]]",
                "[7,[6,[5,[7,0]]]]",
                (None, Some(2)),
            ),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", (None, None)),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                (None, None),
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
                (None, Some(2)),
            ),
        ] {
            let mut result = parse_num(before);
            let (l, r, exploded) = explode(&mut result, 0);
            assert!(exploded);
            assert_eq!(pair, &(l, r));
            assert_eq!(parse_num(after).to_string(), result.to_string());
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(4140, magnitude(sum(&parse_nums(INPUT))));
    }
}
