pub fn part1() -> u64 {
    let input = std::fs::read_to_string("./day3/input.txt").unwrap();

    let mut result: u64 = 0;

    for bank in input.lines() {
        result += find_largest_joltage::<2>(bank) as u64;
    }

    return result;
}

fn find_largest_joltage<const N: usize>(bank: &str) -> u64 {
    let mut it = bank.chars();
    let mut digits = [0u8; N];

    for i in 0..N {
        if let Some(c) = it.next() {
            digits[i] = c.to_digit(10).unwrap() as u8;
        }
    }

    for c in it {
        let mut largest_joltage = get_value(&digits);
        let mut copy = digits.clone();
        let mut largest = digits;
        let digit = c.to_digit(10).unwrap() as u8;

        for i in 0..N {
            for j in 0..N - 1 {
                if j < i {
                    copy[j] = digits[j];
                } else {
                    copy[j] = digits[j + 1];
                }
            }
            copy[N - 1] = digit;

            let new_joltage: u64 = get_value(&copy);
            if new_joltage > largest_joltage {
                largest = copy.clone();
                largest_joltage = new_joltage;
            }
        }
        digits = largest;
    }

    return get_value(&digits);
}

fn get_value<const N: usize>(digits: &[u8; N]) -> u64 {
    let mut value: u64 = 0;
    for i in 0..N {
        value = value * 10 + digits[i] as u64;
    }
    return value;
}

pub fn part2() -> u64 {
    let input = std::fs::read_to_string("./day3/input.txt").unwrap();

    let mut result: u64 = 0;

    for bank in input.lines() {
        result += find_largest_joltage::<12>(bank) as u64;
    }

    return result;
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
        assert_eq!(part2(), 173065202451341);
    }
}
