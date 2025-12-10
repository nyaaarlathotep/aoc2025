use core::panic;
use good_lp::{Expression, Solution, SolverModel, constraint, highs, variable, variables};
use std::collections::HashMap;
use std::vec;

struct Machine {
    target_light: Vec<bool>,
    button: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        let diagram_str = parts[0];
        let target_light = diagram_str
            .trim_matches(|c| c == '[' || c == ']')
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect::<Vec<bool>>();

        let button = parts[1..parts.len() - 1]
            .iter()
            .map(|part| {
                let nums_str = part.trim_matches(|c| c == '(' || c == ')');
                nums_str
                    .split(',')
                    .filter_map(|num_str| num_str.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let joltage_str = parts.last().unwrap();
        let joltage_parts: Vec<&str> = joltage_str
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect();
        let mut joltage = Vec::new();
        for part in joltage_parts {
            if let Ok(num) = part.parse::<usize>() {
                joltage.push(num);
            }
        }

        Some(Machine {
            target_light,
            button,
            joltage,
        })
    }

    fn full_permutations(&self) -> Vec<Vec<usize>> {
        let mut results = vec![];
        let n = self.button.len();
        fn backtrack(
            ind: usize,
            current: &mut Vec<usize>,
            n: usize,
            results: &mut Vec<Vec<usize>>,
        ) {
            if ind == n {
                results.push(current.clone());
                return;
            }
            backtrack(ind + 1, current, n, results);
            current.push(ind);
            backtrack(ind + 1, current, n, results);
            current.pop();
        }
        backtrack(0, &mut vec![], n, &mut results);
        results
    }
}

pub fn part01(input: &str) -> Result<String, &str> {
    let machines = input
        .lines()
        .filter_map(|line| Machine::from_str(line))
        .collect::<Vec<Machine>>();

    let res = machines
        .iter()
        .map(|machine| {
            let mut min_button_permutation: Option<&Vec<usize>> = None;
            println!(
                "Checking machine with target lights: {:?}",
                machine.target_light
            );
            let permutations = machine.full_permutations();
            permutations.iter().for_each(|perm| {
                let mut init_light = vec![false; machine.target_light.len()];
                for &btn_idx in perm {
                    for &light_idx in &machine.button[btn_idx] {
                        init_light[light_idx] = !init_light[light_idx];
                    }
                }
                let mut match_target = true;
                for i in 0..machine.target_light.len() {
                    if init_light[i] != machine.target_light[i] {
                        match_target = false;
                        break;
                    }
                }
                if match_target {
                    if let Some(current_permutation) = min_button_permutation {
                        if perm.len() < current_permutation.len() {
                            min_button_permutation = Some(perm);
                        }
                    } else {
                        min_button_permutation = Some(perm);
                    }
                }
            });
            println!("Found min button permutation: {:?}", min_button_permutation);
            min_button_permutation.unwrap_or(&vec![]).len()
        })
        .sum::<usize>();
    Ok(res.to_string())
}

pub fn part02(input: &str) -> Result<String, &str> {
    let res = input
        .lines()
        .filter_map(|line| Machine::from_str(line))
        .map(|machine| {
            let joltage = machine
                .joltage
                .iter()
                .map(|&j| j as i32)
                .collect::<Vec<i32>>();
            if let Some(count) = solve(&joltage, &machine.button) {
                count
            } else {
                panic!("No solution found");
            }
        })
        .sum::<usize>();
    Ok(res.to_string())
}
/// 求解使得 subsets 组合能恰好消除 target 的最小步数
fn solve(target: &Vec<i32>, subsets: &Vec<Vec<usize>>) -> Option<usize> {
    let mut memo = HashMap::new();
    solve_recursive(target, subsets, &mut memo)
}

fn solve_recursive(
    current_target: &Vec<i32>,
    subsets: &Vec<Vec<usize>>,
    memo: &mut HashMap<Vec<i32>, Option<usize>>,
) -> Option<usize> {
    // 1. 检查备忘录
    if let Some(&res) = memo.get(current_target) {
        return res;
    }

    // 2. Base Case: 检查是否所有目标都已归零
    // 如果所有位都是 0，说明找到了解，步数为 0
    if current_target.iter().all(|&x| x == 0) {
        return Some(0);
    }

    // 3. 剪枝策略：寻找第一个非零的目标索引 (Pivot Index)
    // 我们不需要遍历所有 subsets，只遍历那些能“减少”当前遇到的第一个非零数的 subsets。
    // 这大大减少了搜索空间。
    let first_nonzero_idx = current_target.iter().position(|&x| x > 0);

    match first_nonzero_idx {
        None => {
            // 这里理论上不会到达，因为前面 all(==0) 已经拦截了，但以防万一处理负数情况
            return None;
        }
        Some(idx) => {
            let mut min_steps: Option<usize> = None;

            // 4. 遍历所有操作
            for subset in subsets {
                // 优化：只尝试那些包含 pivot index 的操作
                // 也就是说，如果当前我们在解决 index 0 的剩余数值，我们只看能消除 index 0 的操作
                if !subset.contains(&idx) {
                    continue;
                }

                // 尝试应用这个 subset
                // 检查应用后是否会导致负数（假设我们不允许负数，即必须精确匹配）
                let mut valid = true;
                let mut next_target = current_target.clone();

                for &target_idx in subset {
                    if next_target[target_idx] > 0 {
                        next_target[target_idx] -= 1;
                    } else {
                        // 如果这一步导致某个数变成负数，则此路不通
                        valid = false;
                        break;
                    }
                }

                if valid {
                    // 递归求解剩余部分
                    if let Some(steps) = solve_recursive(&next_target, subsets, memo) {
                        let total_steps = 1 + steps;
                        // 更新最小步数
                        if min_steps.map_or(true, |curr_min| total_steps < curr_min) {
                            min_steps = Some(total_steps);
                        }
                    }
                }
            }

            // 5. 写入备忘录并返回
            memo.insert(current_target.clone(), min_steps);
            min_steps
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    #[test]
    fn test_part1() {
        assert_eq!(part01(&INPUT).unwrap(), "7");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part02(&INPUT).unwrap(), "33");
    }

}
