struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        let x = parts[0].trim().parse::<i64>().ok()?;
        let y = parts[1].trim().parse::<i64>().ok()?;
        Some(Point { x, y })
    }
}

pub fn part01(input: &str) -> Result<String, &str> {
    let points: Vec<Point> = input
        .lines()
        .filter_map(|line| Point::from_str(line))
        .collect();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = (points[i].x - points[j].x + 1) * (points[i].y - points[j].y + 1);
            max_area = if area > max_area { area } else { max_area };
        }
    }

    Ok(max_area.to_string())
}

pub fn part02(input: &str) -> Result<String, &str> {
    Ok("solved part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "50");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
