fn score_invalid(ch: char) -> Option<i32> {
    match ch {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn find_invalid(s: &&str) -> Option<char> {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match (stack.last(), ch) {
            (_, '(' | '[' | '{' | '<') => {
                stack.push(ch);
            }
            (Some('('), ')')
            | (Some('['), ']')
            | (Some('{'), '}')
            | (Some('<'), '>') => {
                stack.pop();
            }
            _ => return Some(ch),
        }
    }
    None
}

fn part1(lines: &[&str]) -> i32 {
    lines.iter().flat_map(find_invalid).flat_map(score_invalid).sum()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let lines = text.lines().collect::<Vec<&str>>();
    println!("{}", part1(&lines));
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
}
