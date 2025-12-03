use std::string;

pub fn part1() -> u64 {
    let input = std::fs::read_to_string("./day3/input.txt").unwrap();

    let mut result: u64 = 0;

    for bank in input.lines() {
        result += find_largest_joltage(bank) as u64;
    }

    return result;
}

fn find_largest_joltage(bank: &str) -> u8 {
    let mut it = bank.chars();

    let mut digits: (u8, u8) = (
        it.next().unwrap().to_digit(10).unwrap() as u8,
        it.next().unwrap().to_digit(10).unwrap() as u8,
    );

    for c in it {
        let current = digits.0 * 10 + digits.1;
        let digit = c.to_digit(10).unwrap() as u8;

        let last_digit = digits.1;

        if digits.0 * 10 + digit > current {
            digits.1 = digit;
        }

        if last_digit * 10 + digit > current {
            digits.0 = last_digit;
            digits.1 = digit;
        }
    }

    return digits.0 * 10 + digits.1;
}

pub fn part2() -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(), 17432);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(), 0);
    }
}
