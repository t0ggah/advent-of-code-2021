use std::str::FromStr;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Clone)]
struct CommandParseError;

impl std::error::Error for CommandParseError {}

impl std::fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "couldn't parse the str in Command")
    }
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((command_part, number_part)) = s.split_once(' ') {
            match (command_part, number_part.parse()) {
                ("forward", Ok(n)) => Ok(Command::Forward(n)),
                ("up", Ok(n)) => Ok(Command::Up(n)),
                ("down", Ok(n)) => Ok(Command::Down(n)),
                _ => Err(Box::new(CommandParseError)),
            }
        } else {
            Err(Box::new(CommandParseError))
        }
    }
}

fn part1(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|x| x.parse::<Command>().unwrap())
        .fold((0, 0), |(px, py), command| match command {
            Command::Forward(n) => (px + n, py),
            Command::Up(n) => (px, py - n),
            Command::Down(n) => (px, py + n),
        })
}

fn part2(input: &str) -> (i32, i32) {
    let (x, y, _aim) = input.lines().map(|x| x.parse::<Command>().unwrap()).fold(
        (0, 0, 0),
        |(x, y, aim), command| match command {
            Command::Up(n) => (x, y, aim - n),
            Command::Down(n) => (x, y, aim + n),
            Command::Forward(n) => (x + n, y + (aim * n), aim),
        },
    );
    (x, y)
}

fn main() {
    let input = include_str!("../input/day2.txt");
    let (hor, vert) = part1(input);
    println!("Result for part 1 was: {} * {} = {}", hor, vert, hor * vert);
    assert_eq!(hor * vert, 2150351);

    let input = include_str!("../input/day2.txt");
    let (hor, vert) = part2(input);
    println!("Result for part 2 was: {} * {} = {}", hor, vert, hor * vert);
    assert_eq!(hor * vert, 1842742223);
}

#[test]
fn test_part1() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    let (horizontal_res, vertical_res) = part1(input);

    assert_eq!(horizontal_res, 15);
    assert_eq!(vertical_res, 10);
}

#[test]
fn test_part2() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    let (horizontal_res, vertical_res) = part2(input);

    assert_eq!(horizontal_res, 15);
    assert_eq!(vertical_res, 60);
}
