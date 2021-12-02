fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let cmds = text
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Command>, _>>()
        .unwrap();
    println!("{}", part1(&cmds));
    println!("{}", part2(&cmds));
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct Location {
    pos: i32,
    depth: i32,
}

impl Location {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            &Command::Forward(dist) => self.pos += dist,
            &Command::Down(dist) => self.depth += dist,
            &Command::Up(dist) => self.depth -= dist,
        }
    }
}

fn part1(cmds: &[Command]) -> i32 {
    let mut loc = Location { pos: 0, depth: 0 };
    for cmd in cmds {
        loc.apply(&cmd);
    }
    loc.pos * loc.depth
}

struct State {
    loc: Location,
    aim: i32,
}

impl State {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            &Command::Forward(dist) => {
                self.loc.pos += dist;
                self.loc.depth += dist * self.aim;
            }
            &Command::Down(dist) => self.aim += dist,
            &Command::Up(dist) => self.aim -= dist,
        }
    }
}

fn part2(cmds: &[Command]) -> i32 {
    let mut state = State { loc: Location { pos: 0, depth: 0 }, aim: 0 };
    for cmd in cmds {
        state.apply(&cmd);
    }
    state.loc.pos * state.loc.depth
}

#[derive(Debug)]
struct ParseError;

impl std::str::FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').ok_or(ParseError)?;
        let dist = dist.parse::<i32>().ok().ok_or(ParseError)?;
        match dir {
            "forward" => Ok(Command::Forward(dist)),
            "down" => Ok(Command::Down(dist)),
            "up" => Ok(Command::Up(dist)),
            _ => Err(ParseError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        let cmds = INPUT
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Command>, _>>()
            .unwrap();
        let result = part1(&cmds);
        assert_eq!(150, result);
    }

    fn test_part2() {
        let cmds = INPUT
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Command>, _>>()
            .unwrap();
        let result = part2(&cmds);
        assert_eq!(901, result);
    }
}
