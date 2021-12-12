use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn explore<'a>(
    g: &'a Graph,
    v: &mut HashMap<&'a str, usize>,
    from: &'a str,
    to: &'a str,
    can_visit: impl Fn(&str, &HashMap<&str, usize>) -> bool + Copy,
) -> usize {
    let mut paths = 0;
    for &nbr in g.get(from).unwrap() {
        if nbr == to {
            paths += 1;
        } else if can_visit(nbr, v) {
            *v.entry(nbr).or_default() += 1;
            paths += explore(g, v, nbr, to, can_visit);
            *v.entry(nbr).or_default() -= 1;
        }
    }
    paths
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(char::is_uppercase)
}

fn part1(g: &Graph) -> usize {
    let mut v = HashMap::new();
    v.insert("start", 1);
    explore(g, &mut v, "start", "end", |s, v| {
        *v.get(s).unwrap_or(&0) == 0 || is_uppercase(s)
    })
}

fn part2(g: &Graph) -> usize {
    let mut v = HashMap::new();
    v.insert("start", 1);
    explore(g, &mut v, "start", "end", |s, v| {
        if is_uppercase(s) || v.get(s).unwrap_or(&0) == &0 {
            return true;
        }
        // visit one small room twice
        let mut lowercase = v.iter().filter(|(s, _)| !is_uppercase(s));
        s != "start" && s != "end" && lowercase.all(|(_, &n)| n < 2)
    })
}

fn parse(s: &str) -> Graph {
    s.lines().fold(Graph::new(), |mut g, line| {
        let (from, to) = line.trim().split_once('-').unwrap();
        g.entry(from).or_default().push(to);
        g.entry(to).or_default().push(from);
        g
    })
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let graph = parse(&text);
    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT1: &'static str = "start-A
                                   start-b
                                   A-c
                                   A-b
                                   b-d
                                   A-end
                                   b-end";

    static INPUT2: &'static str = "dc-end
                                   HN-start
                                   start-kj
                                   dc-start
                                   dc-HN
                                   LN-dc
                                   HN-end
                                   kj-sa
                                   kj-HN
                                   kj-dc";

    static INPUT3: &'static str = "fs-end
                                   he-DX
                                   fs-he
                                   start-DX
                                   pj-DX
                                   end-zg
                                   zg-sl
                                   zg-pj
                                   pj-he
                                   RW-he
                                   fs-DX
                                   pj-RW
                                   zg-RW
                                   start-pj
                                   he-WI
                                   zg-he
                                   pj-fs
                                   start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(10, part1(&parse(INPUT1)));
        assert_eq!(19, part1(&parse(INPUT2)));
        assert_eq!(226, part1(&parse(INPUT3)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36, part2(&parse(INPUT1)));
        assert_eq!(103, part2(&parse(INPUT2)));
        assert_eq!(3509, part2(&parse(INPUT3)));
    }
}
