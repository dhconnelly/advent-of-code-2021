fn pair_match(a: char, b: char) -> bool {
    matches!((a, b), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

fn score_invalid(ch: char) -> i64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn find_invalid(s: &&str) -> Option<char> {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match (stack.last(), ch) {
            (_, '(' | '[' | '{' | '<') => {
                stack.push(ch);
            }
            (Some(&a), b) if pair_match(a, b) => {
                stack.pop();
            }
            _ => return Some(ch),
        }
    }
    None
}

fn part1(lines: &[&str]) -> i64 {
    lines.iter().flat_map(find_invalid).map(score_invalid).sum()
}

fn make_completion(line: &&str) -> Vec<char> {
    line.chars().fold(Vec::new(), |mut stack, ch| {
        if stack.is_empty() || matches!(ch, '(' | '[' | '{' | '<') {
            stack.push(ch);
        } else {
            stack.pop();
        }
        stack
    })
}

fn score_completion(completion: Vec<char>) -> i64 {
    completion.iter().rev().fold(0, |score, ch| {
        let val = match *ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!(),
        };
        5 * score + val
    })
}

fn part2(lines: &[&str]) -> i64 {
    let mut scores = lines
        .iter()
        .filter(|s| find_invalid(s).is_none())
        .map(make_completion)
        .map(score_completion)
        .collect::<Vec<i64>>();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let lines = text.lines().collect::<Vec<&str>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn test_part1() {
        let lines = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(26397, part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(288957, part2(&lines));
    }
}
