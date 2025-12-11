use std::collections::{HashMap, HashSet};

pub fn part01(input: &str) -> Result<String, &str> {
    let m = parse(input);
    let res = paths("you", "out", &mut HashSet::new(), &m);
    // eprintln!("paths:{:?}",res);
    Ok(res.len().to_string())
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (left, right) = line.split_once(": ").unwrap();
        let vals = if right.is_empty() {
            Vec::new()
        } else {
            right.split_whitespace().collect::<Vec<&str>>()
        };
        map.insert(left, vals);
    }
    map
}

fn paths<'a>(
    now: &'a str,
    end: &'a str,
    visited: &mut HashSet<&'a str>,
    m: &'a HashMap<&str, Vec<&str>>,
) -> Vec<Vec<&'a str>> {
    if visited.contains(now) {
        return vec![];
    }
    if now == end {
        return vec![vec![end]];
    }
    visited.insert(now);
    let mut res = vec![];
    if let Some(nexts) = m.get(now) {
        for &next in nexts {
            let succeed_paths = paths(next, end, visited, m);
            for mut path in succeed_paths {
                path.push(now);
                res.push(path);
            }
        }
    }
    visited.remove(now);
    res
}

pub fn part02(input: &str) -> Result<String, &str> {
    let m = parse(input);
    let start1 = part2_paths("svr", "fft", vec![], &m);
    let mut res = vec![];
    for start_path in start1 {
        eprintln!("{:?}", start_path);
        let mid_paths = part2_paths("fft", "dac", start_path, &m);
        for mid in mid_paths {
            eprintln!("{:?}", mid);
            let end_paths = part2_paths("dac", "out", mid, &m);
            for path in end_paths {
                eprintln!("{:?}", path);
                res.push(path);
            }
        }
    }
    eprintln!("-----------------");
    let start2 = part2_paths("svr", "dac", vec![], &m);
    for start_path in start2 {
        eprintln!("{:?}", start_path);
        let mid_paths = part2_paths("dac", "fft", start_path, &m);
        for mid in mid_paths {
            eprintln!("{:?}", mid);
            let end_paths = part2_paths("fft", "out", mid, &m);
            for path in end_paths {
                eprintln!("{:?}", path);
                res.push(path);
            }
        }
    }
    // let res = part2_paths("svr", "out", vec![], &m);
    eprintln!("paths:{:?}", res);
    Ok(res.len().to_string())
}
fn part2_paths<'a>(
    now: &'a str,
    end: &'a str,
    path: Vec<&'a str>,
    m: &'a HashMap<&str, Vec<&str>>,
) -> Vec<Vec<&'a str>> {
    // if path.contains(&now) {
    //     return vec![];
    // }
    if now == end {
        return vec![path];
    }
    let mut res = vec![];
    if let Some(nexts) = m.get(now) {
        for &next in nexts {
            let mut new_path = path.clone();
            new_path.push(now);
            let succeed_paths = part2_paths(next, end, new_path, m);
            for path in succeed_paths {
                res.push(path);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "5");
    }
    const PART2_INPUT: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    #[test]
    fn test_part2() {
        assert_eq!(part02(&PART2_INPUT).unwrap(), "2");
    }
}
