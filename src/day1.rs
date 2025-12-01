pub fn part1() -> i32 {
    let input = std::fs::read_to_string("./day1/input.txt").unwrap();

    let mut result = 0;
    let mut pos: i32 = 50;

    for line in input.lines() {
        let rot = line.chars().next().unwrap();
        let rest = &line[1..];
        let number = i32::from_str_radix(rest, 10).unwrap();

        pos += match rot {
            'L' => -(number),
            'R' => number,
            _ => panic!("Invalid rotation"),
        };

        if pos % 100 == 0 {
            result += 1;
        }
    }

    return result;
}

pub fn part2() -> i32 {
    let input = std::fs::read_to_string("./day1/input.txt").unwrap();

    let mut result = 0;
    let mut pos: i32 = 50;

    for line in input.lines() {
        let rot = line.chars().next().unwrap();
        let rest = &line[1..];
        let mut number = i32::from_str_radix(rest, 10).unwrap();

        pos = pos % 100;
        pos = if pos < 0 { 100 + pos } else { pos };
        let start = pos;
        result += number / 100;
        number %= 100;

        pos += match rot {
            'L' => -(number),
            'R' => number,
            _ => panic!("Invalid rotation"),
        };

        if start != 0 && pos <= 0 || pos >= 100 {
            result += 1;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(), 1129);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(), 6638);
    }
}
