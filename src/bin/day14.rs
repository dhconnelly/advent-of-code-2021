use std::collections::HashMap;

type Pair = (u8, u8);
type Freqs = HashMap<u8, i64>;

fn merge(a: Freqs, b: Freqs) -> Freqs {
    b.iter().fold(a, |mut acc, (&k, v)| {
        *acc.entry(k).or_default() += v;
        acc
    })
}

fn expand_freqs(
    input @ (left, right): Pair,
    rules: &HashMap<Pair, u8>,
    n: usize,
    memo: &mut HashMap<(Pair, usize), Freqs>,
) -> Freqs {
    if n == 0 {
        return Freqs::new();
    } else if let Some(freqs) = memo.get(&(input, n)) {
        return freqs.clone();
    }
    let &mid = rules.get(&input).unwrap();
    let mut freqs = merge(
        expand_freqs((left, mid), rules, n - 1, memo),
        expand_freqs((mid, right), rules, n - 1, memo),
    );
    *freqs.entry(mid).or_default() += 1;
    memo.entry((input, n)).or_insert(freqs).clone()
}

fn solve(input: &str, rules: &HashMap<Pair, u8>, n: usize) -> i64 {
    // populate initial freqs from input string
    let freqs = input.bytes().fold(Freqs::new(), |mut acc, ch| {
        *acc.entry(ch).or_default() += 1;
        acc
    });

    // fold together recursively expanded freqs for each pair in the input
    let mut memo = HashMap::new();
    let pairs = input.bytes().zip(input.bytes().skip(1));
    let freqs = pairs.fold(freqs, |acc, pair| {
        merge(acc, expand_freqs(pair, rules, n, &mut memo))
    });

    let most_common = freqs.values().max().unwrap();
    let least_common = freqs.values().min().unwrap();
    most_common - least_common
}

fn parse(s: &str) -> (String, HashMap<Pair, u8>) {
    let mut lines = s.lines();
    let input = lines.next().unwrap().trim().to_string();
    lines.next().unwrap(); // skip empty line
    let rules = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(s, t)| (s.trim().as_bytes(), t.trim().as_bytes()))
        .map(|(s, t)| ((s[0], s[1]), t[0]))
        .collect();
    (input, rules)
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (input, rules) = parse(&text);
    println!("{}", solve(&input, &rules, 10));
    println!("{}", solve(&input, &rules, 40));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test() {
        let (input, rules) = parse(INPUT);
        println!("{}, {:?}", input, rules);
        assert_eq!(1588, solve(&input, &rules, 10));
        assert_eq!(2188189693529, solve(&input, &rules, 40));
    }
}
