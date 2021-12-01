use std::str::FromStr;

fn part1(input: &str) -> u32 {
    let numbers = parse_lines::<u32>(input);

    count_increases(numbers)
}

fn part2(input: &str) -> u32 {
    let numbers = parse_lines::<u32>(input);
    let windows = numbers.windows(3).map(|x| x.iter().sum()).collect();

    count_increases(windows)
}

fn count_increases(vec: Vec<u32>) -> u32 {
    vec.windows(2).fold(
        0,
        |sum, window| if window[0] < window[1] { sum + 1 } else { sum },
    )
}

fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input.lines().map(|x| x.parse::<T>().unwrap()).collect()
}

fn main() {
    let input = include_str!("../input/day1.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 1288);

    let input = include_str!("../input/day1.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 1311);
}

#[test]
fn test_part_1() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let result = part1(input);
    assert_eq!(result, 7)
}

#[test]
fn test_part_2() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let result = part2(input);
    assert_eq!(result, 5)
}
