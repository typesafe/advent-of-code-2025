pub fn part1() -> u64 {
    let input = std::fs::read_to_string("./day2/input.txt").unwrap();

    let mut result = 0;

    for range in input.split(',') {
        let (mut number, end) = {
            let mut splits = range.trim().split('-');
            let s = u64::from_str_radix(splits.next().unwrap(), 10).unwrap();
            let e = u64::from_str_radix(splits.next().unwrap(), 10).unwrap();
            (s, e)
        };

        while number <= end {
            if consists_of_repeated_digits(number) {
                result += number;
            }
            number += 1;
        }
    }

    return result;
}

fn consists_of_repeated_digits(number: u64) -> bool {
    let s = number.to_string();

    if s.len() % 2 == 1 {
        return false;
    }

    return s[0..s.len() / 2] == s[s.len() / 2..s.len()];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        assert_eq!(part1(), 30323879646);
    }
}
