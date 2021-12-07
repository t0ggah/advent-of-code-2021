fn part1(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let min = crabs.iter().min().copied().unwrap();
    let max = crabs.iter().max().copied().unwrap();
    let mut current_min_fuel: Option<(i32, i32)> = None;
    for i in min..=max {
        let fuel = crabs.iter().map(|x| (x - i).abs()).sum();
        if let Some(min_fuel) = current_min_fuel {
            if min_fuel.1 > fuel {
                current_min_fuel = Some((i, fuel));
            }
        } else {
            current_min_fuel = Some((i, fuel));
        }
    }

    current_min_fuel.unwrap().1
}

fn part2(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let min = crabs.iter().min().copied().unwrap();
    let max = crabs.iter().max().copied().unwrap();
    let mut current_min_fuel: Option<i32> = None;
    for i in min..=max {
        let fuel = crabs
            .iter()
            .map(|x| {
                let n = (x - i).abs();

                (n * (n + 1)) / 2
            })
            .sum();

        if let Some(min_fuel) = current_min_fuel {
            if min_fuel > fuel {
                current_min_fuel = Some(fuel);
            }
        } else {
            current_min_fuel = Some(fuel);
        }
    }

    current_min_fuel.unwrap()
}

fn main() {
    let input = include_str!("../input/day7.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 348996);

    let input = include_str!("../input/day7.txt");
    let result = part2(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 98231647);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = part1(input);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_partj() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = part2(input);
        assert_eq!(result, 168);
    }
}
