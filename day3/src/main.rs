fn part1(input: &str) -> u32 {
    let value = |x: char| if x == '1' { 1 } else { 0 };

    let ones_count = input
        .lines()
        .map(|x| x.chars().map(value).collect::<Vec<u32>>())
        .reduce(|counts, bytes| counts.iter().zip(bytes).map(|(c, b)| c + b).collect())
        .unwrap();

    let input_length = input.lines().count();
    let mut gamma_rate: u32 = 0;
    for (index, count) in ones_count.iter().enumerate() {
        if (*count as usize) > (input_length / 2) {
            gamma_rate =
                gamma_rate | 1u32.rotate_left((ones_count.len() - index - 1).try_into().unwrap());
        }
    }

    let mut epsilon_rate: u32 = 0;
    for (index, count) in ones_count.iter().enumerate() {
        if (*count as usize) < (input_length / 2) {
            epsilon_rate =
                epsilon_rate | 1u32.rotate_left((ones_count.len() - index - 1).try_into().unwrap());
        }
    }

    epsilon_rate as u32 * gamma_rate as u32
}

fn part2(input: &str) -> u32 {
    let left = input
        .lines()
        .map(|x| (x, x.chars().map(|c| c.to_digit(10).unwrap()).collect()))
        .collect::<Vec<(&str, Vec<u32>)>>();

    let find_rating = |most_common: bool| {
        let mut left = left.clone();
        let mut current_position: usize = 0;

        loop {
            let ones_count = left
                .iter()
                .fold(0u32, |count, item| count + item.1[current_position]);

            let bit_to_filter = if ones_count as f64 >= (left.len() as f64 / 2f64) {
                if most_common {
                    1
                } else {
                    0
                }
            } else {
                if most_common {
                    0
                } else {
                    1
                }
            };

            left.retain(|x| x.1[current_position] == bit_to_filter);

            if left.len() < 2 {
                let answer = left.first().expect("Should be one correct answer");

                break u32::from_str_radix(answer.0, 2)
                    .expect("wrong input, expected only 0 and 1");
            }
            current_position += 1;
        }
    };

    let oxygen_generator_rating = find_rating(true);
    let co2_scrubber_rating = find_rating(false);

    co2_scrubber_rating * oxygen_generator_rating
}

fn main() {
    let input = include_str!("../input/day3.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 845186);

    let input = include_str!("../input/day3.txt");
    let result = part2(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 4636702);
}

#[test]
fn test_part1() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    let result = part1(input);

    assert_eq!(result, 198);
}

#[test]
fn test_part2() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    let result = part2(input);

    assert_eq!(result, 230);
}
