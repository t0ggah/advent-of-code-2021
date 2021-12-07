fn part1(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let fuel_cost = |column: i32| {
        return move |x: &i32| (x - column).abs();
    };
    let min_column = crabs.iter().min().copied().unwrap();
    let max_column = crabs.iter().max().copied().unwrap();
    let max_column_fuel = crabs.iter().map(fuel_cost(max_column)).sum();
    (min_column..max_column).fold(max_column_fuel, |min_fuel, column| {
        let fuel = crabs.iter().map(fuel_cost(column)).sum();
        min_fuel.min(fuel)
    })
}

fn part2(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let fuel_cost = |column: i32| {
        return move |x: &i32| {
            let n = (x - column).abs();

            (n * (n + 1)) / 2
        };
    };

    let min_column = crabs.iter().min().copied().unwrap();
    let max_column = crabs.iter().max().copied().unwrap();
    let max_column_fuel = crabs.iter().map(fuel_cost(max_column)).sum();
    (min_column..max_column).fold(max_column_fuel, |min_fuel, column| {
        let fuel = crabs.iter().map(fuel_cost(column)).sum();
        min_fuel.min(fuel)
    })
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
