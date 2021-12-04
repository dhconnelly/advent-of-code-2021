#[derive(Debug)]
enum State {
    Unmarked,
    Marked,
}

#[derive(Debug)]
struct Square {
    value: i32,
    state: State,
}

type IndicesTable = std::collections::HashMap<i32, Vec<(usize, usize)>>;

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
    indices: IndicesTable,
}

fn parse_order(s: &str) -> Vec<i32> {
    s.split(',')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap()
}

fn indices_table(squares: &Vec<Vec<Square>>) -> IndicesTable {
    let mut table = IndicesTable::new();
    for (i, row) in squares.iter().enumerate() {
        for (j, sq) in row.iter().enumerate() {
            table.entry(sq.value).or_default().push((i, j));
        }
    }
    table
}

fn parse_board(s: &str) -> Board {
    let parse_tok = |tok: &str| Square {
        value: tok.parse().unwrap(),
        state: State::Unmarked,
    };
    let parse_line =
        |line: &str| line.split_whitespace().map(parse_tok).collect();
    let squares = s.lines().map(parse_line).collect();
    let indices = indices_table(&squares);
    Board { squares, indices }
}

#[derive(Debug)]
struct Game {
    order: Vec<i32>,
    boards: Vec<Board>,
}

fn parse_game(s: &str) -> Game {
    let mut segs = s.split("\n\n");
    let order = parse_order(segs.next().unwrap());
    let boards = segs.map(parse_board).collect::<Vec<Board>>();
    Game { order, boards }
}

fn main() {
    let path = std::env::args().nth(1).expect("missing input path");
    let text = std::fs::read_to_string(&path).unwrap();
    let game = parse_game(&text);
    println!("game: {:?}", game);
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_part1() {
        let game = parse_game(INPUT);
        println!("{:?}", game);
    }
}
