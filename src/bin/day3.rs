fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let width = text.lines().next().unwrap().len();
    let nums = text
        .lines()
        .map(|row| u64::from_str_radix(row, 2).unwrap())
        .collect::<Vec<u64>>();
    println!("{}", part1(width, &nums));
    println!("{}", part2(width, &nums));
}

fn part1(width: usize, nums: &[u64]) -> u64 {
    let mut gamma = 0;
    for bit in most_common_bits(width, nums) {
        gamma <<= 1;
        if bit == MostCommon::One {
            gamma += 1;
        }
    }
    let epsilon = !gamma & ((1 << width) - 1);
    return gamma * epsilon;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MostCommon {
    Tie,
    Zero,
    One,
}

fn most_common_bits(width: usize, nums: &[u64]) -> Vec<MostCommon> {
    let mut one_counts = vec![0; width];
    for num in nums {
        for i in 0..width {
            if num & (1 << (width - i - 1)) > 0 {
                one_counts[i] += 1;
            }
        }
    }
    let mut most_common = vec![MostCommon::Tie; width];
    for (i, count) in one_counts.iter().enumerate() {
        if *count > nums.len() - *count {
            most_common[i] = MostCommon::One;
        } else if *count < nums.len() - *count {
            most_common[i] = MostCommon::Zero;
        }
    }
    most_common
}

fn part2(width: usize, nums: &[u64]) -> u64 {
    let oxy = find_num(width, nums.into(), |most_common, bit_index, bit| {
        match most_common[bit_index] {
            MostCommon::Tie | MostCommon::One => bit == 1,
            MostCommon::Zero => bit == 0,
        }
    });
    let co2 = find_num(width, nums.into(), |most_common, bit_index, bit| {
        match most_common[bit_index] {
            MostCommon::Tie | MostCommon::One => bit == 0,
            MostCommon::Zero => bit == 1,
        }
    });
    oxy * co2
}

fn find_num(
    width: usize,
    mut nums: Vec<u64>,
    bit_pred: impl Fn(&[MostCommon], usize, u8) -> bool,
) -> u64 {
    for i in 0..width {
        if nums.len() == 1 {
            break;
        }
        let most_common = most_common_bits(width, &nums);
        nums.retain(|row| {
            let bit = ((row & (1 << (width - i - 1))) > 0) as bool as u8;
            bit_pred(&most_common, i, bit)
        });
    }
    *nums.first().unwrap()
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
        let width = INPUT.lines().next().unwrap().len();
        let nums = INPUT
            .lines()
            .map(|row| u64::from_str_radix(row, 2).unwrap())
            .collect::<Vec<u64>>();
        assert_eq!(198, part1(width, &nums));
    }

    #[test]
    fn test_part2() {
        let width = INPUT.lines().next().unwrap().len();
        let nums = INPUT
            .lines()
            .map(|row| u64::from_str_radix(row, 2).unwrap())
            .collect::<Vec<u64>>();
        assert_eq!(230, part2(width, &nums));
    }
}
