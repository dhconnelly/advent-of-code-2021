fn parse(s: &str) -> (i64, i64) {
    let get_pos = |line: &str| (line.as_bytes().last().unwrap() - b'0') as i64;
    let mut lines = s.lines();
    let pos1 = get_pos(lines.next().unwrap());
    let pos2 = get_pos(lines.next().unwrap());
    (pos1, pos2)
}

fn part1(mut pos1: i64, mut pos2: i64) -> i64 {
    let (mut score1, mut score2) = (0, 0);
    let (mut next_roll, mut rolls) = (1, 0);
    loop {
        pos1 = (((pos1 - 1) + (3 * next_roll + 3)) % 10) + 1;
        next_roll = (next_roll + 3) % 10;
        rolls += 3;
        score1 += pos1;
        if score1 >= 1000 {
            return score2 * rolls;
        }

        pos2 = (((pos2 - 1) + (3 * next_roll + 3)) % 10) + 1;
        next_roll = (next_roll + 3) % 10;
        rolls += 3;
        score2 += pos2;

        if score2 >= 1000 {
            return score1 * rolls;
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (pos1, pos2) = parse(&text);
    println!("{}", part1(pos1, pos2));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part1() {
        let (pos1, pos2) = parse(INPUT);
        assert_eq!((4, 8), (pos1, pos2));
    }
}
