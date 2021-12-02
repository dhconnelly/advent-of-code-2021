fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let cmds = text
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Command>, _>>()
        .unwrap();

    let init = Location { pos: 0, depth: 0 };
    println!("{}", apply(init.clone(), &cmds));
    println!("{}", apply(State { loc: init.clone(), aim: 0 }, &cmds));
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

trait Commandable {
    fn apply(&mut self, cmd: &Command);
    fn result(&self) -> i32;
}

fn apply(mut t: impl Commandable, cmds: &[Command]) -> i32 {
    for cmd in cmds {
        t.apply(cmd);
    }
    t.result()
}

#[derive(Clone, Copy)]
struct Location {
    pos: i32,
    depth: i32,
}

impl Commandable for Location {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            &Command::Forward(dist) => self.pos += dist,
            &Command::Down(dist) => self.depth += dist,
            &Command::Up(dist) => self.depth -= dist,
        }
    }

    fn result(&self) -> i32 {
        self.pos * self.depth
    }
}

struct State {
    loc: Location,
    aim: i32,
}

impl Commandable for State {
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

    fn result(&self) -> i32 {
        self.loc.result()
    }
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

    fn cmds() -> Vec<Command> {
        INPUT
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Command>, _>>()
            .unwrap()
    }

    #[test]
    fn test_part1() {
        let init = Location { pos: 0, depth: 0 };
        let result = apply(init, &cmds());
        assert_eq!(150, result);
    }

    #[test]
    fn test_part2() {
        let init = Location { pos: 0, depth: 0 };
        let result = apply(State { loc: init, aim: 0 }, &cmds());
        assert_eq!(900, result);
    }
}
