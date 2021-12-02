fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let nums = text
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    println!("{}", part1(&nums));
    println!("{}", part2(&nums));
}

fn part1(nums: &[i32]) -> usize {
    nums.iter()
        .zip(nums.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

fn part2(nums: &[i32]) -> usize {
    (0..nums.len() - 3)
        .filter(|&i| {
            let w1 = &nums[i..i + 3];
            let w2 = &nums[i + 1..i + 4];
            w1.iter().sum::<i32>() < w2.iter().sum()
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increases = part1(input);
        assert_eq!(7, increases);
    }

    #[test]
    fn test_part2() {
        let input = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increases = part2(input);
        assert_eq!(5, increases);
    }
}
