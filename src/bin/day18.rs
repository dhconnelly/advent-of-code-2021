use std::fmt;
use std::str;

#[derive(Debug, Clone)]
enum Num {
    Regular(u64),
    Nested(Box<Num>),
    Pair(Box<Num>, Box<Num>),
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Regular(num) => write!(f, "{}", num)?,
            Num::Nested(num) => write!(f, "{}", num)?,
            Num::Pair(lhs, rhs) => write!(f, "[{}, {}]", lhs, rhs)?,
        }
        Ok(())
    }
}

fn add(lhs: Num, rhs: Num) -> Num {
    Num::Pair(Box::new(lhs), Box::new(rhs))
}

fn sum(nums: &[Num]) -> Num {
    nums.iter().for_each(|num| println!("{}", num));
    let sum = nums.iter().skip(1).cloned().fold(nums[0].clone(), add);
    println!("sum = {}", sum);
    sum
}

fn magnitude(num: Num) -> u64 {
    0
}

fn parse(chars: &mut impl Iterator<Item = char>) -> Num {
    match chars.next() {
        None => panic!("unexpected eof"),
        Some('[') => {
            let lhs = parse(chars);
            let pair = if let Some(',') = chars.next() {
                let rhs = parse(chars);
                assert_eq!(Some(']'), chars.next());
                Num::Pair(Box::new(lhs), Box::new(rhs))
            } else {
                assert_eq!(Some(']'), chars.next());
                Num::Nested(Box::new(lhs))
            };
            pair
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

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let nums = parse_nums(&text);
    println!("{}", magnitude(sum(&nums)));
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
        let expected: Num = parse(
            &mut "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .chars(),
        );
        assert_eq!(expected.to_string(), sum(&nums).to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(4140, magnitude(sum(&parse_nums(INPUT))));
    }
}
