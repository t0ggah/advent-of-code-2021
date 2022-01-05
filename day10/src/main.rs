enum LineCheckResult {
    Ok,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn check_lines(line: &str) -> LineCheckResult {
    let mut opening = Vec::new();
    for char in line.trim().chars() {
        match char {
            '(' => opening.push(')'),
            '[' => opening.push(']'),
            '{' => opening.push('}'),
            '<' => opening.push('>'),
            ')' | '}' | ']' | '>' => {
                if let Some(popped) = opening.pop() {
                    if popped != char {
                        return LineCheckResult::Corrupted(char);
                    }
                }
            }
            _ => unreachable!("other include any other characters"),
        }
    }

    if opening.is_empty() {
        return LineCheckResult::Ok;
    }

    return LineCheckResult::Incomplete(opening);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            if let LineCheckResult::Corrupted(char) = check_lines(line) {
                return Some(char);
            }

            return None;
        })
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut result = input
        .lines()
        .filter_map(|line| {
            if let LineCheckResult::Incomplete(chars) = check_lines(line) {
                return Some(chars);
            }

            return None;
        })
        .map(|x| {
            x.iter()
                .rev()
                .map(|x| match x {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!("other include any other characters"),
                })
                .fold(0, |total, item| total * 5 + item)
        })
        .collect::<Vec<u64>>();
    result.sort();

    // Always an odd number of items
    let middle_index = result.len() / 2;

    result[middle_index]
}

fn main() {
    let input = include_str!("../input/day10.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 413733);

    let input = include_str!("../input/day10.txt");
    let result = part2(input);
    println!("Result for part 2 was: {}", result);
    assert_eq!(result, 3354640192);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let result = part1(input);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let result = part2(input);
        assert_eq!(result, 288957);
    }
}
