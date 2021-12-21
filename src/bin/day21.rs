use std::collections::HashMap;

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

fn play_quantum(
    pos1: i64,
    score1: i64,
    pos2: i64,
    score2: i64,
    mem: &mut HashMap<(i64, i64, i64, i64), (i64, i64)>,
) -> (i64, i64) {
    if score1 >= 21 {
        (1, 0)
    } else if score2 >= 21 {
        (0, 1)
    } else if let Some(wins) = mem.get(&(pos1, score1, pos2, score2)) {
        *wins
    } else {
        let (mut wins1, mut wins2) = (0, 0);
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let npos1 = ((pos1 - 1) + roll1 + roll2 + roll3) % 10 + 1;
                    let nscore1 = score1 + npos1;
                    let wins = play_quantum(pos2, score2, npos1, nscore1, mem);
                    wins1 += wins.1;
                    wins2 += wins.0;
                }
            }
        }
        mem.insert((pos1, score1, pos2, score2), (wins1, wins2));
        (wins1, wins2)
    }
}

fn part2(pos1: i64, pos2: i64) -> i64 {
    let (wins1, wins2) = play_quantum(pos1, 0, pos2, 0, &mut HashMap::new());
    wins1.max(wins2)
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let (pos1, pos2) = parse(&text);
    println!("{}", part1(pos1, pos2));
    println!("{}", part2(pos1, pos2));
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
        assert_eq!(739785, part1(pos1, pos2));
    }

    #[test]
    fn test_part2() {
        assert_eq!((1, 0), play_quantum(1, 26, 1, 0, &mut HashMap::new()));
        assert_eq!((0, 1), play_quantum(1, 0, 1, 26, &mut HashMap::new()));
        assert_eq!((1, 0), play_quantum(1, 26, 1, 0, &mut HashMap::new()));
        assert_eq!((0, 1), play_quantum(1, 0, 1, 26, &mut HashMap::new()));

        let mut mem = HashMap::new();
        assert_eq!((27, 0), play_quantum(7, 20, 1, 0, &mut mem));
        assert_eq!(&(27, 0), mem.get(&(7, 20, 1, 0)).unwrap());

        let mut mem = HashMap::from([((1, 2, 3, 4), (7, 14))]);
        assert_eq!((7, 14), play_quantum(1, 2, 3, 4, &mut mem));

        let (pos1, pos2) = parse(INPUT);
        assert_eq!(444356092776315, part2(pos1, pos2));
    }
}
