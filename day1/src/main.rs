use std::str::FromStr;

fn part1(input: &'static str) -> u32 {
    let numbers = parse_lines::<u32>(input);

    count_increases(numbers)
}

fn part2(input: &'static str) -> u32 {
    let numbers = parse_lines::<u32>(input).collect::<Vec<_>>();
    let windows = (0..numbers.len() - 2).map(|i| numbers[i..i + 3].into_iter().sum());

    count_increases(windows)
}

fn count_increases(mut iter: impl Iterator<Item = u32>) -> u32 {
    let mut count = 0;
    let mut previous_number = iter.next().unwrap();
    for number in iter {
        if number > previous_number {
            count += 1;
        }
        previous_number = number;
    }

    count
}

fn parse_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input.lines().map(|x| x.parse::<T>().unwrap())
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
