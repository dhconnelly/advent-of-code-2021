use std::collections::HashMap;

fn frequencies(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}

fn apply(input: String, rules: &mut HashMap<String, String>) -> String {
    if input.len() == 1 {
        return input;
    }
    if let Some(output) = rules.get(&input) {
        return output.to_string();
    }
    let x = input.len() / 2;
    let (l, m, r) = (&input[..x], &input[x - 1..x + 1], &input[x..]);
    let (l, r) = (apply(l.to_string(), rules), apply(r.to_string(), rules));
    let cx = rules.get(m).unwrap().as_bytes()[1] as char;
    let output = format!("{}{}{}", l, cx, r);
    rules.insert(input.to_string(), output.clone());
    output
}

fn solve(input: &str, rules: &mut HashMap<String, String>, n: usize) -> usize {
    let output = (0..n).fold(input.to_string(), |s, _| apply(s, rules));
    let freqs = frequencies(&output);
    let most_common = freqs.values().max().unwrap();
    let least_common = freqs.values().min().unwrap();
    most_common - least_common
}

fn parse(s: &str) -> (String, HashMap<String, String>) {
    let mut lines = s.lines();
    let input = lines.next().unwrap().trim().to_string();
    lines.next().unwrap();
    let rules = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(s, t)| {
            let s = s.trim();
            let (sb, tb) = (s.as_bytes(), t.trim().as_bytes());
            let (a, b, c) = (sb[0] as char, tb[0] as char, sb[1] as char);
            (s.to_string(), format!("{}{}{}", a, b, c))
        })
        .collect();
    (input, rules)
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (input, mut rules) = parse(&text);
    println!("{}", solve(&input, &mut rules, 10));
    //println!("{}", solve(&input, &mut rules, 40));
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
    fn test_part1() {
        let (input, mut rules) = parse(INPUT);
        println!("{}, {:?}", input, rules);
        assert_eq!(1588, solve(&input, &mut rules, 10));
        //assert_eq!(2188189693529, solve(&input, &mut rules, 40));
    }
}
