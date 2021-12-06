fn parse(s: &str) -> Vec<i64> {
    let mut fish = vec![0; 9];
    for i in s.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        fish[i] += 1;
    }
    fish
}

fn step(fish: &mut [i64], next_fish: &mut [i64]) {
    next_fish[8] = fish[0];
    for i in 1..=8 {
        next_fish[i - 1] = fish[i];
    }
    next_fish[6] += fish[0];
}

fn simulate(mut fish: Vec<i64>, steps: usize) -> i64 {
    let mut next_fish = vec![0; fish.len()];
    for _ in 0..steps {
        step(&mut fish, &mut next_fish);
        for i in 0..fish.len() {
            fish[i] = next_fish[i];
        }
    }
    fish.iter().sum::<i64>()
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let fish = parse(&text);
    println!("{}", simulate(fish.clone(), 80));
    println!("{}", simulate(fish.clone(), 256));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let fish = parse(INPUT);
        let result = simulate(fish, 80);
        assert_eq!(5934, result);
    }

    #[test]
    fn test_part2() {
        let fish = parse(INPUT);
        let result = simulate(fish, 256);
        assert_eq!(26984457539, result);
    }
}
