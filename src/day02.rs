use std::collections::HashSet;
use std::{f32::consts::E, fmt::Error};

pub fn part01(input: &str) -> Result<String, Error> {
    let res: i64 = input
        .split(",")
        .map(|line| line.split('-').collect::<Vec<&str>>())
        .map(|parts| {
            let mut candidates = vec![];
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            if end < start {
                return vec![];
            }
            let start_len = parts[0].len();
            let end_len = parts[1].len();
            for prefix_len in start_len / 2..=end_len / 2 {
                if prefix_len == 0 {
                    continue;
                }
                let mut prefix = "1".to_string();
                prefix.push_str("0".repeat(prefix_len - 1).as_str());
                loop {
                    let candidate: i64 = {
                        let candidate_str = prefix.repeat(2);
                        let candidate: i64 = candidate_str.parse().unwrap();
                        candidate
                    };
                    if candidate >= start && candidate <= end {
                        candidates.push(candidate);
                    }
                    if candidate > end {
                        break;
                    }
                    let next_slice = add_one(&prefix);
                    if next_slice.len() > prefix.len() {
                        break;
                    }

                    prefix = next_slice;
                }
            }
            candidates
        })
        .flatten()
        .sum();
    Ok(res.to_string())
}

fn add_one(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut carry = 1;
    for i in (0..chars.len()).rev() {
        if carry == 0 {
            break;
        }
        let digit = chars[i].to_digit(10).unwrap() + carry;
        if digit >= 10 {
            chars[i] = std::char::from_digit(digit - 10, 10).unwrap();
            carry = 1;
        } else {
            chars[i] = std::char::from_digit(digit, 10).unwrap();
            carry = 0;
        }
    }
    if carry == 1 {
        chars.insert(0, '1');
    }
    chars.iter().collect()
}

pub fn part02(input: &str) -> Result<String, Error> {
    let ranges = get_ranges(input);
    let mut result: i64 = 0;

    for (n1, n2) in ranges {
        for n in n1..=n2 {
            let s = n.to_string();

            for i in 0..(s.len() / 2) {
                let pattern = &s[0..=i];

                if s.len() % pattern.len() != 0 {
                    continue;
                }
                let repeats = s.len() / pattern.len();
                if s == pattern.repeat(repeats) {
                    result += i64::from(n);
                    break;
                }
            }
        }
    }
    Ok(result.to_string())
}
fn get_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|r| r.split_once("-").unwrap())
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part1() {
        // 读取同目录下的 test 文件

        assert_eq!(part01(&INPUT).unwrap(), "1227775554");
    }
    #[test]
    fn test_part2() {
        // 读取同目录下的 test 文件

        assert_eq!(part02(&INPUT).unwrap(), "4174379265");
    }
}
