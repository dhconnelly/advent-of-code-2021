fn most_common_bits(rows: &[&str]) -> Vec<u8> {
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
    let mut most_common = vec![0; rows[0].len()];
    for (i, count) in counts.iter().enumerate() {
        if *count > rows.len() / 2 {
            most_common[i] = 1;
        }
    }
    most_common
}

fn part1(most_common: &[u8]) -> u64 {
    let mut gamma = 0;
    for bit in most_common {
        gamma <<= 1;
        gamma += *bit as u64;
    }
    let epsilon = !gamma & ((1 << most_common.len()) - 1);
    return gamma * epsilon;
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let rows = text.lines().collect::<Vec<&str>>();
    let most_common = most_common_bits(&rows);
    println!("{}", part1(&most_common));
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
        let most_common = most_common_bits(&rows);
        assert_eq!(198, part1(&most_common));
    }
}
