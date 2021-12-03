fn part1(rows: &[&str]) -> u64 {
    let mut counts = vec![0; rows[0].len()];
    for row in rows {
        for (i, bit) in row.chars().enumerate() {
            counts[i] += match bit {
                '1' => Some(1),
                '0' => Some(0),
                _ => None,
            }
            .unwrap()
        }
    }
    let mut gamma = 0;
    for count in counts {
        gamma <<= 1;
        if count > rows.len() / 2 {
            gamma += 1;
        }
    }
    let epsilon = !gamma & ((1 << rows[0].len()) - 1);
    return gamma * epsilon;
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let rows = text.lines().collect::<Vec<&str>>();
    println!("{}", part1(&rows));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let rows = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(198, part1(&rows));
    }
}
