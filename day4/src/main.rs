#[derive(Debug)]
struct Board {
    won: bool,
    values: Vec<(bool, u8)>,
}

impl Board {
    fn new() -> Self {
        Self {
            won: false,
            values: Vec::new(),
        }
    }

    fn add_row(&mut self, row: &mut Vec<(bool, u8)>) {
        self.values.append(row);
    }

    fn has_won(&self) -> bool {
        self.won
    }

    fn add_drawed_value(&mut self, val: u8) {
        let number_pos = self.values.iter().position(|(_, num)| val == *num);
        if let Some(pos) = number_pos {
            if let Some(elem) = self.values.get_mut(pos) {
                (*elem).0 = true;
            }
        }
    }

    fn check(&mut self) {
        if self.won {
            return;
        }

        for row in 0..5 {
            if self
                .values
                .iter()
                .skip(row * 5)
                .take(5)
                .all(|(checked, _)| *checked)
            {
                self.won = true;
                break;
            }
        }
        if self.won {
            return;
        }

        for column in 0..5 {
            if self
                .values
                .iter()
                .skip(column)
                .step_by(5)
                .all(|(checked, _)| *checked)
            {
                self.won = true;
                break;
            }
        }
    }

    fn remaining_values(&self) -> u32 {
        self.values
            .iter()
            .filter_map(|(check, val)| if *check { None } else { Some(*val as u32) })
            .sum()
    }
}

fn part1(input: &str) -> u32 {
    let (values, mut boards) = parse_values_and_board(input);

    for value in values.iter() {
        for board in boards.iter_mut() {
            board.add_drawed_value(*value);
            board.check();

            if board.has_won() {
                return board.remaining_values() * *value as u32;
            }
        }
    }

    unreachable!("A board should win");
}

fn part2(input: &str) -> u32 {
    let (values, mut boards) = parse_values_and_board(input);

    let mut values_iter = values.iter();
    let mut current_value = values_iter.next().unwrap();
    let mut last_result = 0;
    loop {
        for board in boards.iter_mut() {
            board.add_drawed_value(*current_value);
            board.check();

            if board.has_won() {
                last_result = board.remaining_values() * *current_value as u32;
            }
        }

        boards.retain(|b| !b.has_won());

        if boards.is_empty() {
            return last_result;
        }

        current_value = values_iter.next().unwrap();
    }
}

fn parse_values_and_board(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut input_iter = input.lines();

    let values = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>();

    let mut boards = Vec::new();
    let mut current_board: Board = Board::new();
    for line in input_iter.skip(1) {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::new();
        } else {
            let mut row = line
                .split_whitespace()
                .map(|x| (false, x.parse().unwrap()))
                .collect::<Vec<(bool, u8)>>();
            current_board.add_row(&mut row);
        }
    }
    boards.push(current_board);

    (values, boards)
}

fn main() {
    let input = include_str!("../input/day4.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 35670);

    let input = include_str!("../input/day4.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 22704);
}

#[test]
fn test_part1() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

    let result = part1(input);

    assert_eq!(result, 4512);
}

#[test]
fn test_part2() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

    let result = part2(input);

    assert_eq!(result, 1924);
}
