use std::collections::HashMap;

fn part1(words: &HashMap<String, i32>) -> usize {
    words.len()
}

fn part2(words: &HashMap<String, i32>) -> i32 {
    *words.values().max().unwrap()
}

fn parse(input: &str) -> HashMap<String, i32> {
    let mut words = HashMap::new();
    for word in input.split(char::is_whitespace).filter(|w| !w.is_empty()) {
        *words.entry(word.to_string()).or_insert(0) += 1;
    }
    words
}

pub fn run(input: &str) {
    let words = parse(input);
    println!("{}", part1(&words));
    println!("{}", part2(&words));
}
