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

pub fn part2() -> u64 {
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
            if consists_of_repeated_digits2(number) {
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

fn consists_of_repeated_digits2(number: u64) -> bool {
    let s = number.to_string();

    let mut repetitions = 2;
    let mut result = false;

    while repetitions <= s.len() {
        if s.len() % repetitions == 0 {
            let len = s.len() / repetitions;
            let first = s[0..len].to_string();
            let mut match_all = true;

            for i in 1..repetitions {
                if s[i * len..(i + 1) * len] != first {
                    match_all = false;
                    break;
                }
            }

            if match_all {
                result = true;
                break;
            }
        } else {
            repetitions += 1;
            continue;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        assert_eq!(part1(), 30323879646);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(part2(), 43872163557);
    }
}
