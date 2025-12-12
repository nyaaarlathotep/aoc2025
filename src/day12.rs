use core::panic;

struct Gift {
    area: usize,
    shape: Vec<Vec<char>>,
}

struct Region {
    width: usize,
    length: usize,
    gift_index: Vec<usize>,
}
impl Gift {
    // Parses the ASCII shape block of the gift (excluding the ID line)
    fn from_str(s: &str) -> Option<Gift> {
        if s.trim().is_empty() {
            return None;
        }

        let shape: Vec<Vec<char>> = s
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Calculate area by counting '#'
        let area = shape
            .iter()
            .flat_map(|row: &Vec<char>| row.iter())
            .filter(|&&c| c == '#')
            .count();

        Some(Gift { area, shape })
    }
}

impl Region {
    // Parses a line like "4x4: 0 0 0 0 2 0"
    fn from_str(s: &str) -> Option<Region> {
        let (dimensions, indices) = s.split_once(':')?;
        let (w_str, l_str) = dimensions.split_once('x')?;

        let width: usize = w_str.trim().parse().ok()?;
        let length = l_str.trim().parse().ok()?;

        let gift_index: Vec<usize> = indices
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        Some(Region {
            width,
            length,
            gift_index,
        })
    }
}

fn parse(input: &str) -> (Vec<Gift>, Vec<Region>) {
    let mut gifts = Vec::new();
    let mut regions = Vec::new();

    for block in input.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if block.contains('x') && block.contains(':') {
            for line in block.lines() {
                if let Some(r) = Region::from_str(line) {
                    regions.push(r);
                }
            }
        } else {
            if let Some((_header, shape_str)) = block.split_once('\n') {
                if let Some(g) = Gift::from_str(shape_str) {
                    gifts.push(g);
                }
            }
        }
    }

    (gifts, regions)
}

pub fn part01(input: &str) -> Result<String, &str> {
    let (gifts, regions) = parse(input);
    let res = regions
        .iter()
        .filter(|&region| {
            let gift_area = region
                .gift_index
                .iter()
                .enumerate()
                .map(|(i, &count)| gifts[i].area * count)
                .sum::<usize>();

            if gift_area > region.length * region.width {
                return false;
            }
            let gift_count = region.gift_index.iter().sum::<usize>();
            let overflow_count = (region.length / 3) * (region.width / 3);
            if overflow_count>=gift_count{
                return true;
            }

            panic!("couldn't handle");
        })
        .count();
    Ok(res.to_string())
}

pub fn part02(input: &str) -> Result<String, &str> {
    Ok("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "2");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
