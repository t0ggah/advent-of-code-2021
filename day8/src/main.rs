use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| x.trim().split_once(" | "))
        .flat_map(|(_, digits)| digits.trim().split_whitespace())
        .filter(|x| match x.chars().count() {
            2 => true,
            4 => true,
            3 => true,
            7 => true,
            _ => false,
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| x.trim().split_once(" | "))
        .fold(0, |sum, (signal, digits)| {
            let signal_pattern = signal.split_whitespace().collect::<Vec<&str>>();
            let digits_with_sorted_chars = digits
                .split_whitespace()
                .map(|d| {
                    let mut sorted_chars = d.chars().collect::<Vec<char>>();
                    sorted_chars.sort();
                    sorted_chars.into_iter().collect::<String>()
                })
                .collect::<Vec<String>>();
            let pattern = find_signal_pattern(signal_pattern);

            sum + digits_with_sorted_chars.iter().rev().enumerate().fold(
                0,
                |line_sum, (i, digit)| {
                    let digit = pattern[digit];
                    line_sum + digit * 10usize.pow(i as u32)
                },
            )
        })
}

fn find_signal_pattern(signal: Vec<&str>) -> HashMap<String, usize> {
    let mut sorted_signals = signal
        .iter()
        .map(|x| {
            let mut vec = x.trim().chars().collect::<Vec<char>>();
            vec.sort();
            vec.iter().collect::<String>()
        })
        .collect::<Vec<String>>();

    sorted_signals.sort_by(|x1, x2| x1.len().cmp(&x2.len()));

    let mut indices: [usize; 10] = [1, 7, 4, 0, 0, 0, 0, 0, 0, 8];
    let one_signal = &sorted_signals[0];
    let seven_signal = &sorted_signals[1];
    let four_signal = &sorted_signals[2];

    for i in 3..6 {
        let signal = &sorted_signals[i];
        let intersection_seven_count = intersection_count(signal, seven_signal);
        let intersection_four_count = intersection_count(signal, four_signal);
        let value = match (intersection_four_count, intersection_seven_count) {
            (2, _) => 2,
            (_, 3) => 3,
            _ => 5,
        };

        indices[i] = value;
    }

    for i in 6..9 {
        let signal = &sorted_signals[i];
        let intersection_four_count = intersection_count(signal, four_signal);
        let intersection_one_count = intersection_count(signal, one_signal);

        let value = match (intersection_four_count, intersection_one_count) {
            (4, _) => 9,
            (_, 1) => 6,
            _ => 0,
        };

        indices[i] = value;
    }

    sorted_signals
        .into_iter()
        .zip(indices.iter().copied())
        .collect()
}

fn intersection_count(str1: &str, str2: &str) -> usize {
    let str1: HashSet<char> = str1.chars().collect();
    let str2: HashSet<char> = str2.chars().collect();

    str1.intersection(&str2).count()
}

fn main() {
    let input = include_str!("../input/day8.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 264);

    let input = include_str!("../input/day8.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 1063760);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let result = part1(input);

        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let result = part2(input);

        assert_eq!(result, 5353);
    }
}
