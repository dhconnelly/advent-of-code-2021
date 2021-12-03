fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let rows = text.lines().collect::<Vec<&str>>();
    println!("{}", part1(&rows));
    println!("{}", part2(&rows));
}

fn part1(rows: &[&str]) -> u64 {
    let s = (0..rows[0].len()).map(|i| match most_common(i, rows) {
        MostCommon::One | MostCommon::Tie => '1',
        MostCommon::Zero => '0',
    });
    let gamma = u64::from_str_radix(&s.collect::<String>(), 2).unwrap();
    let epsilon = !gamma & ((1 << rows[0].len()) - 1);
    return gamma * epsilon;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MostCommon {
    Tie,
    Zero,
    One,
}

fn most_common(i: usize, rows: &[&str]) -> MostCommon {
    let ones = rows.iter().filter(|row| row.as_bytes()[i] == b'1').count();
    if ones == rows.len() - ones {
        MostCommon::Tie
    } else if ones < rows.len() - ones {
        MostCommon::Zero
    } else {
        MostCommon::One
    }
}

fn part2(rows: &[&str]) -> u64 {
    let s = find_row(rows.into(), |mcb, bit| match mcb {
        MostCommon::Tie | MostCommon::One => bit == b'1',
        MostCommon::Zero => bit == b'0',
    });
    let t = find_row(rows.into(), |mcb, bit| match mcb {
        MostCommon::Tie | MostCommon::One => bit == b'0',
        MostCommon::Zero => bit == b'1',
    });
    let oxy = u64::from_str_radix(s, 2).unwrap();
    let co2 = u64::from_str_radix(t, 2).unwrap();
    oxy * co2
}

fn find_row(
    mut rows: Vec<&str>,
    bit_pred: impl Fn(MostCommon, u8) -> bool,
) -> &str {
    for i in 0..rows[0].len() {
        if rows.len() == 1 {
            break;
        }
        let mcb = most_common(i, &rows);
        rows.retain(|row| bit_pred(mcb, row.as_bytes()[i]));
    }
    *rows.first().unwrap()
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

    #[test]
    fn test_part2() {
        let rows = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(230, part2(&rows));
    }
}
