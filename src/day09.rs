use std::f32::consts::E;

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

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            if (p1.x == p2.x) || (p1.y == p2.y) {
                // line
                if !no_intersect(
                    &Line {
                        p1: p1.clone(),
                        p2: p2.clone(),
                    },
                    &lines,
                ) {
                    continue;
                }
                let area = (points[i].x - points[j].x + 1) * (points[i].y - points[j].y + 1);
                max_area = if area > max_area { area } else { max_area };
            } else {
                // square
                let line1 = Line {
                    p1: p1.clone(),
                    p2: Point { x: p2.x, y: p1.y },
                };
                if !no_intersect(&line1, &lines) {
                    continue;
                }

                let line2 = Line {
                    p1: Point { x: p2.x, y: p1.y },
                    p2: p2.clone(),
                };
                if !no_intersect(&line2, &lines) {
                    continue;
                }
                let line3 = Line {
                    p1: p2.clone(),
                    p2: Point { x: p1.x, y: p2.y },
                };
                if !no_intersect(&line3, &lines) {
                    continue;
                }
                let line4 = Line {
                    p1: Point { x: p1.x, y: p2.y },
                    p2: p1.clone(),
                };
                if !no_intersect(&line4, &lines) {
                    continue;
                }
                let area = (points[i].x - points[j].x + 1) * (points[i].y - points[j].y + 1);
                max_area = if area > max_area { area } else { max_area };
            }
        }
    }

    Ok(max_area.to_string())
}

fn no_intersect(line: &Line, lines: &Vec<Line>) -> bool {
    for l in lines {
        if intersect(line, l) {
            return false;
        }
    }
    true
}

fn intersect(line1: &Line, line2: &Line) -> bool {
    if line1.p1.x == line1.p2.x && line2.p1.x == line2.p2.x {
        return false; // both vertical
    }
    if line1.p1.y == line1.p2.y && line2.p1.y == line2.p2.y {
        return false; // both horizontal
    }
    if line1.p1.x == line1.p2.x {
        // line1 vertical line2 horizontal
        let line1_x = line1.p1.x;
        let line1_y_min = line1.p1.y.min(line1.p2.y);
        let line1_y_max = line1.p1.y.max(line1.p2.y);
        let line2_x_min = line2.p1.x.min(line2.p2.x);
        let line2_x_max = line2.p1.x.max(line2.p2.x);
        if line1_x < line2_x_min || line1_x > line2_x_max {
            return false;
        }
        let line_2_y = line2.p1.y;
        if line_2_y <= line1_y_min || line_2_y >= line1_y_max {
            return false;
        }
        true
    } else {
        // line1 horizontal line2 vertical
        let line1_y = line1.p1.y;
        let line1_x_min = line1.p1.x.min(line1.p2.x);
        let line1_x_max = line1.p1.x.max(line1.p2.x);
        let line2_y_min = line2.p1.y.min(line2.p2.y);
        let line2_y_max = line2.p1.y.max(line2.p2.y);
        if line1_y < line2_y_min || line1_y > line2_y_max {
            return false;
        }
        let line_2_x = line2.p1.x;
        if line_2_x <= line1_x_min || line_2_x >= line1_x_max {
            return false;
        }
        true
    }
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
