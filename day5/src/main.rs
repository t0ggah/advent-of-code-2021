use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    str::FromStr,
};

#[derive(Debug)]
struct LineParseError;

impl Display for LineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't parse string into Line")
    }
}

impl std::error::Error for LineParseError {}

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.start.1 == self.end.1
    }

    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn covered_points<'a>(&'a self) -> Box<dyn Iterator<Item = (u32, u32)> + 'a> {
        if self.is_horizontal() {
            return Box::new(
                (min(self.start.1, self.end.1)..=max(self.start.1, self.end.1))
                    .map(|x| (self.start.0, x)),
            );
        }
        if self.is_vertical() {
            return Box::new(
                (min(self.start.0, self.end.0)..=max(self.start.0, self.end.0))
                    .map(|x| (x, self.start.1)),
            );
        }

        let get_range = |start, end| -> Box<dyn Iterator<Item = u32>> {
            if start < end {
                Box::new(start..=end)
            } else {
                Box::new((end..=start).rev())
            }
        };

        let x_values = get_range(self.start.0, self.end.0);
        let y_values = get_range(self.start.1, self.end.1);

        return Box::new(x_values.zip(y_values));
    }
}

impl FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once(" -> ") {
            if let (Some((start_x, start_y)), Some((end_x, end_y))) =
                (start.split_once(','), end.split_once(','))
            {
                return Ok(Self {
                    start: (start_x.parse()?, start_y.parse()?),
                    end: (end_x.parse()?, end_y.parse()?),
                });
            }
        }

        Err(Box::new(LineParseError))
    }
}

fn part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.parse::<Line>().unwrap())
        .filter(|l| l.horizontal_or_vertical());

    count_covered_points(lines)
}

fn part2(input: &str) -> u32 {
    let lines = input.lines().map(|x| x.parse::<Line>().unwrap());

    count_covered_points(lines)
}

fn count_covered_points(lines: impl Iterator<Item = Line>) -> u32 {
    let mut map = HashMap::new();
    for line in lines {
        let points = line.covered_points();

        for point in points {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    map.values().filter(|x| **x > 1).count() as u32
}

fn main() {
    let input = include_str!("../input/day5.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 5169);

    let input = include_str!("../input/day5.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 22083);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, 5)
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, 12)
    }
}
