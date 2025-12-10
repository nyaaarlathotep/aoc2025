#[derive(Clone)]
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

    fn rect_area_with(&self, other: &Point) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        width * height
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

struct Line {
    p1: Point,
    p2: Point,
}

pub fn part02(input: &str) -> Result<String, &str> {
    let points = input
        .lines()
        .filter_map(|line| Point::from_str(line))
        .collect::<Vec<Point>>();
    let mut lines = Vec::new();
    for i in 0..points.len() {
        lines.push(Line {
            p1: points[i].clone(),
            p2: points[(i + 1) % points.len()].clone(),
        });
    }
    let mut possible_rects: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points[i + 1..]
                .iter()
                .map(move |p2| (p1, p2, p1.rect_area_with(p2)))
        })
        .collect();

    possible_rects.sort_by(|a, b| b.2.cmp(&a.2));

    let res = possible_rects
        .into_iter()
        .find(|&(p1, p2, _)| {
            lines.iter().all(|line| {
                let left = p1.x.max(p2.x) <= line.p1.x.min(line.p2.x);
                let right = p1.x.min(p2.x) >= line.p1.x.max(line.p2.x);
                let above = p1.y.max(p2.y) <= line.p1.y.min(line.p2.y);
                let below = p1.y.min(p2.y) >= line.p1.y.max(line.p2.y);
                left || right || above || below
            })
        })
        .expect("not find")
        .2;
    Ok(res.to_string())
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
        assert_eq!(part02(&INPUT).unwrap(), "24");
    }
}
