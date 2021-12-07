fn emulate_lanterfish_and_count(input: &str, days: u32) -> usize {
    let fishes = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let mut collection: [usize; 9] = [0; 9];

    for fish in &fishes {
        collection[*fish] += 1;
    }

    for _ in 0..days {
        let reset = collection[0];
        collection[0] = 0;

        for i in 1..9 {
            collection[i - 1] = collection[i];
        }

        collection[6] += reset;
        collection[8] = reset;
    }

    collection.iter().sum()
}

fn part1(input: &str) -> usize {
    emulate_lanterfish_and_count(input, 80)
}

fn part2(input: &str) -> usize {
    emulate_lanterfish_and_count(input, 256)
}

fn main() {
    let input = include_str!("../input/day6.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 345387);

    let input = include_str!("../input/day6.txt");
    let result = part2(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 1574445493136);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3,4,3,1,2";
        let result = part1(input);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_part2() {
        let input = "3,4,3,1,2";
        let result = part2(input);
        assert_eq!(result, 26984457539);
    }
}
