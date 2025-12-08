use std::{cmp::Reverse, collections::HashMap};
#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

// ---------------------- 适配 Point 的并查集 ----------------------
struct UnionFind {
    // key: 当前节点, value: 父节点
    parents: HashMap<Point, Point>,
    // key: 根节点, value: 该连通分量的大小
    sizes: HashMap<Point, usize>,
    // 当前连通分量的总数
    count: usize,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parents: HashMap::new(),
            sizes: HashMap::new(),
            count: 0,
        }
    }

    /// 注册一个点。如果点已存在，忽略；如果不存在，初始化为独立集合
    fn add(&mut self, p: Point) {
        if !self.parents.contains_key(&p) {
            self.parents.insert(p.clone(), p.clone());
            self.sizes.insert(p, 1);
            self.count += 1;
        }
    }

    /// 查找根节点 (带路径压缩)
    fn find(&mut self, p: &Point) -> Point {
        // 如果点不存在，先自动注册
        if !self.parents.contains_key(p) {
            self.add(p.clone());
            return p.clone();
        }

        let mut path = Vec::new();
        let mut root = p.clone();

        // 1. 寻找根节点
        while let Some(parent) = self.parents.get(&root) {
            if parent == &root {
                break;
            }
            path.push(root.clone()); // 记录路径以便稍后压缩
            root = parent.clone();
        }

        // 2. 路径压缩：将路径上所有点直接指向根
        for node in path {
            self.parents.insert(node, root.clone());
        }

        root
    }

    /// 合并两个点所在的集合
    fn union(&mut self, p1: &Point, p2: &Point) {
        let root1 = self.find(p1);
        let root2 = self.find(p2);

        if root1 == root2 {
            return;
        }

        // 按大小合并
        let size1 = self.sizes[&root1];
        let size2 = self.sizes[&root2];

        if size1 < size2 {
            self.parents.insert(root1.clone(), root2.clone());
            self.sizes.insert(root2.clone(), size1 + size2);
            self.sizes.remove(&root1);
        } else {
            self.parents.insert(root2.clone(), root1.clone());
            self.sizes.insert(root1.clone(), size1 + size2);
            self.sizes.remove(&root2);
        }

        self.count -= 1;
    }

    /// 获取最终结果：所有连通分量及其大小
    fn get_components_info(&self) -> Vec<(usize, Point)> {
        // 返回 (大小, 根节点示例)
        self.sizes
            .iter()
            .map(|(root, &size)| (size, root.clone()))
            .collect()
    }
    fn get_node_numbers(&self) -> usize {
        self.parents.len()
    }
    fn get_component_count(&self) -> usize {
        self.count
    }
}

impl Point {
    fn from_str(s: &str) -> Point {
        let coords: Vec<i64> = s
            .split(',')
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

#[derive(Eq, PartialEq, Debug)] // Eq 是 Ord 的前提
struct PointPair<'a> {
    p1: &'a Point,
    p2: &'a Point,
    dist: i64,
}

impl Ord for PointPair<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for PointPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part01(input: &str) -> Result<String, &str> {
    Ok(part01_solve(1000, input))
}

fn part01_solve(edge_count: usize, input: &str) -> String {
    let points: Vec<Point> = input.lines().map(|line| Point::from_str(line)).collect();
    let mut min_heap = std::collections::BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].distance(&points[j]);
            let pp: PointPair<'_> = PointPair {
                p1: &points[i],
                p2: &points[j],
                dist,
            };
            min_heap.push(pp);
            if min_heap.len() > edge_count {
                min_heap.pop();
            }
        }
    }

    let mut uf = UnionFind::new();

    for _ in 0..edge_count {
        let pp = min_heap.pop().unwrap();
        // println!(
        //     "Point1: ({},{},{}), Point2: ({},{},{}), DistSq: {}",
        //     pp.p1.x, pp.p1.y, pp.p1.z, pp.p2.x, pp.p2.y, pp.p2.z, pp.dist
        // );
        uf.union(pp.p1, pp.p2);
    }

    let mut components = uf.get_components_info();
    components.sort_by(|a, b| b.0.cmp(&a.0));
    let mut res = 1;
    for i in 0..3 {
        let (size, _) = &components[i];
        // println!(
        //     "Largest Component Root: ({},{},{}), Size: {}",
        //     root.x, root.y, root.z, size
        // );
        res = res * size;
    }

    res.to_string()
}

pub fn part02(input: &str) -> Result<String, &str> {
    let points: Vec<Point> = input.lines().map(|line| Point::from_str(line)).collect();
    let mut min_heap = std::collections::BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = points[i].distance(&points[j]);
            let pp: PointPair<'_> = PointPair {
                p1: &points[i],
                p2: &points[j],
                dist,
            };
            min_heap.push(Reverse(pp));
        }
    }

    let mut uf = UnionFind::new();
    while let Some(Reverse(pp)) = min_heap.pop() {
        // println!(
        //     "point1: ({},{},{}), Point2: ({},{},{}) DistSq: {}",
        //     pp.p1.x, pp.p1.y, pp.p1.z, pp.p2.x, pp.p2.y, pp.p2.z, pp.dist
        // );
        uf.union(pp.p1, pp.p2);
        // let components = uf.get_components_info();
        // println!("Current component count: {}", components.len());

        if uf.get_component_count() == 1 && uf.get_node_numbers() == points.len() {
            let components = uf.get_components_info();
            let (size, _) = &components[0];
            if size == &points.len() {
                // println!(
                //     "ppoint1: ({},{},{}), Point2: ({},{},{})",
                //     pp.p1.x, pp.p1.y, pp.p1.z, pp.p2.x, pp.p2.y, pp.p2.z,
                // );
                return Ok((pp.p1.x * pp.p2.x).to_string());
            }
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    #[test]
    fn test_part1() {
        assert_eq!(part01_solve(10, &INPUT), "40");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "25272");
    }
}
