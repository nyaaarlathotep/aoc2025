use std::fmt::Error;

pub fn part01(input: &str) -> Result<String, Error> {
    let (raw_ranges, numbers) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(raw_ranges);
    let merged_ranges = merge_ranges(&ranges);

    let res = numbers
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .filter(|&n| {
            for (start, end) in &merged_ranges {
                if n >= *start && n <= *end {
                    return true;
                }
            }
            false
        })
        .count()
        .to_string();

    Ok(res.to_string())
}

fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();
            (start, end)
        })
        .collect()
}

fn merge_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(i64, i64)> = vec![];
    for range in sorted_ranges {
        if merged.is_empty() {
            merged.push(range);
        } else {
            let last = merged.last_mut().unwrap();
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged.push(range);
            }
        }
    }
    merged
}

pub fn part02(input: &str) -> Result<String, Error> {
    let (raw_ranges, _) = input.split_once("\n\n").unwrap();
    let ranges = parse_ranges(raw_ranges);
    let merged_ranges = merge_ranges(&ranges);

    Ok(merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<i64>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "3");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "14");
    }
}
