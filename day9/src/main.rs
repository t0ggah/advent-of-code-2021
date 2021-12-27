use std::{collections::HashSet, ops::Add, str::FromStr};

struct Map {
    board: Vec<Vec<u32>>,
    visited: HashSet<Point>,
}

impl Map {
    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn visit(&mut self, point: Point) {
        self.visited.insert(point);
    }

    fn has_visited(&mut self, point: Point) -> bool {
        self.visited.contains(&point)
    }

    fn value(&self, point: Point) -> u32 {
        self.board[point.y() as usize][point.x() as usize]
    }

    fn inside_map(&self, point: &Point) -> bool {
        (0..self.width() as isize).contains(&point.x())
            && (0..self.height() as isize).contains(&point.y())
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height_map = s
            .lines()
            .map(|x| x.trim().chars().map(|x| x.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>();
        Ok(Self {
            board: height_map,
            visited: HashSet::new(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl Point {
    fn all_deltas() -> [Point; 4] {
        [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)]
    }

    fn x(&self) -> isize {
        self.0
    }

    fn y(&self) -> isize {
        self.1
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn part1(input: &str) -> u32 {
    let height_map = Map::from_str(input).unwrap();
    let lowest_points = find_lowest_points(&height_map);

    lowest_points.iter().map(|(x, _)| x + 1).sum()
}

fn part2(input: &str) -> u32 {
    let mut height_map = Map::from_str(input).unwrap();
    let lowest_points = find_lowest_points(&height_map);

    let mut basins: Vec<u32> = lowest_points
        .iter()
        .map(|(_, point)| find_basin_sum(&mut height_map, *point))
        .collect();

    basins.sort_by(|a, b| b.cmp(a));

    basins
        .into_iter()
        .take(3)
        .reduce(|mul, val| mul * val)
        .unwrap()
}

fn find_basin_sum(map: &mut Map, point: Point) -> u32 {
    if !map.inside_map(&point) {
        return 0;
    }

    if map.has_visited(point) {
        return 0;
    }

    if map.value(point) == 9 {
        return 0;
    }

    map.visit(point);

    return 1 + Point::all_deltas()
        .iter()
        .map(|x| find_basin_sum(map, x + &point))
        .sum::<u32>();
}

fn find_lowest_points(height_map: &Map) -> Vec<(u32, Point)> {
    let mut lowest_points = Vec::new();
    for y in 0..height_map.height() {
        for x in 0..height_map.width() {
            let current_point = Point(x as isize, y as isize);
            let current_height = height_map.value(current_point);

            if Point::all_deltas()
                .iter()
                .map(|dir| &current_point + dir)
                .filter(|p| height_map.inside_map(p))
                .all(|p| current_height < height_map.value(p))
            {
                lowest_points.push((current_height, current_point));
            }
        }
    }

    lowest_points
}

fn main() {
    let input = include_str!("../input/day9.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 562);

    let input = include_str!("../input/day9.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 1076922);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let result = part1(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let result = part2(input);
        assert_eq!(result, 1134);
    }
}
