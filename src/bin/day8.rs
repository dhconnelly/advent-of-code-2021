use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Digit(u8);

impl Digit {
    fn len(&self) -> u32 {
        self.0.count_ones()
    }

    fn contains(self, other: Digit) -> bool {
        self.0 & other.0 == other.0
    }
}

struct Entry {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

fn parse_digit(s: &str) -> Digit {
    Digit(s.bytes().fold(0u8, |acc, ch| acc | (1 << (ch - b'a'))))
}

fn parse(line: &str) -> Entry {
    let (input, output) = line.split_once(" | ").unwrap();
    let input = input.split(' ').map(parse_digit).collect();
    let output = output.split(' ').map(parse_digit).collect();
    Entry { input, output }
}

fn assign(entry: &Entry) -> HashMap<Digit, usize> {
    let mut map = HashMap::new();
    let mut rev = [Digit(0); 10];

    // pass 1: unique signal lengths
    for digit in entry.input.iter() {
        let val = match digit.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            _ => continue,
        };
        map.insert(*digit, val);
        rev[val] = *digit;
    }

    // pass 2: unique based on first pass
    for digit in entry.input.iter() {
        if map.contains_key(digit) {
            continue;
        }
        let val = match digit.len() {
            6 if digit.contains(rev[4]) => 9,
            6 if digit.contains(rev[1]) => 0,
            6 => 6,
            5 if digit.contains(rev[1]) => 3,
            _ => continue,
        };
        map.insert(*digit, val);
        rev[val] = *digit;
    }

    // pass 3: distinguish 2 vs. 5 with second pass info
    for digit in entry.input.iter() {
        if map.contains_key(digit) {
            continue;
        }
        let val = if rev[6].contains(*digit) { 5 } else { 2 };
        map.insert(*digit, val);
    }

    map
}

fn entry_value(entry: &Entry) -> usize {
    let map = assign(&entry);
    let digit_value = |digit: &Digit| map.get(digit).unwrap().to_string();
    let output_value: String = entry.output.iter().map(digit_value).collect();
    output_value.parse().unwrap()
}

fn part2(entries: &[Entry]) -> usize {
    entries.iter().map(entry_value).sum()
}

fn part1(entries: &[Entry]) -> usize {
    let is1478 = |digit: &&Digit| matches!(digit.len(), 2 | 3 | 4 | 7);
    let count1478 = |entry: &Entry| entry.output.iter().filter(is1478).count();
    entries.iter().map(count1478).sum()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let entries = text.lines().map(parse).collect::<Vec<Entry>>();
    println!("{}", part1(&entries));
    println!("{}", part2(&entries));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1() {
        let entries = INPUT.lines().map(parse).collect::<Vec<Entry>>();
        assert_eq!(26, part1(&entries));
    }

    #[test]
    fn test_part2() {
        let entries = INPUT.lines().map(parse).collect::<Vec<Entry>>();
        assert_eq!(61229, part2(&entries));
    }
}
