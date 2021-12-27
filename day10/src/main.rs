fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut opening = Vec::new();
            for char in line.trim().chars() {
                match char {
                    '(' => opening.push(')'),
                    '[' => opening.push(']'),
                    '{' => opening.push('}'),
                    '<' => opening.push('>'),
                    ')' => {
                        if let Some(popped) = opening.pop() {
                            if popped != ')' {
                                return Some(char);
                            }
                        }
                    }
                    ']' => {
                        if let Some(popped) = opening.pop() {
                            if popped != ']' {
                                return Some(char);
                            }
                        }
                    }
                    '}' => {
                        if let Some(popped) = opening.pop() {
                            if popped != '}' {
                                return Some(char);
                            }
                        }
                    }
                    '>' => {
                        if let Some(popped) = opening.pop() {
                            if popped != '>' {
                                return Some(char);
                            }
                        }
                    }
                    _ => panic!("Shouldn't reach this"),
                }
            }
            None
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

fn main() {
    let input = include_str!("../input/day10.txt");
    let result = part1(input);
    println!("Result for part 1 was: {}", result);
    assert_eq!(result, 413733);
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
}
