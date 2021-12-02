fn part1(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|x| {
            let mut line_iter = x.split(' ');

            let command = line_iter.next().unwrap();
            let number = line_iter.next().unwrap().parse().unwrap();

            match (command, number) {
                ("forward", n) => (n, 0),
                ("up", n) => (0, 0 - n),
                ("down", n) => (0, n),
                _ => (0, 0),
            }
        })
        .fold((0, 0), |(px, py), (x, y)| (px + x, py + y))
}

fn part2(input: &str) -> (i32, i32) {
    let (x, y, _aim) = input
        .lines()
        .map(|x| {
            let mut line_iter = x.split(' ');

            let command = line_iter.next().unwrap();
            let number = line_iter.next().unwrap().parse().unwrap();

            match (command, number) {
                ("forward", n) => (n, 0),
                ("up", n) => (0, 0 - n),
                ("down", n) => (0, n),
                _ => (0, 0),
            }
        })
        .fold((0, 0, 0), |(px, py, pa), (x, y)| match (x, y) {
            (0, n) => (px, py, pa + n),
            (n, 0) => (px + n, py + (pa * n), pa),
            _ => (px, py, pa),
        });
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
