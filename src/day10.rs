use core::panic;
use good_lp::{Expression, Solution, SolverModel, constraint, highs, variable, variables};
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
fn solve(
    target_ints: &Vec<i32>,
    subsets: &Vec<Vec<usize>>,
) -> Option<usize> {
    // 1. 初始化变量构建器
    let mut vars = variables!();

    // 2. 创建主要决策变量 x_i (每个子集取多少个)
    // 依然保持整数约束 (Integer Programming)
    let x_vars: Vec<_> = (0..subsets.len())
        .map(|i| vars.add(variable().min(0).integer().name(format!("x{}", i))))
        .collect();

    // 4. 目标函数: 最小化 x 的总和
    let objective: Expression = x_vars.iter().sum();

    // 5. 初始化模型
    let mut problem = vars.minimise(objective.clone()).using(highs);

    // 6. 生成约束 (仅保留数值累加)
    for pos in 0..target_ints.len() {
        let target_val = target_ints[pos];

        // --- 找出覆盖当前 pos 的 x 变量 ---
        let relevant_vars: Vec<Expression> = subsets
            .iter()
            .enumerate()
            .filter(|(_, subset)| subset.contains(&pos))
            .map(|(i, _)| x_vars[i].into()) // 将 x_var 转换为 Expression
            .collect();

        if relevant_vars.is_empty() {
            // 如果没有任何子集能影响这个位置，且目标值不为0，则无解
            if target_val > 0 {
                eprintln!("❌ 位置 {} 无法被覆盖 (无相关子集且目标值 > 0)", pos);
                return None;
            }
        } else {
            let sum_expr: Expression = relevant_vars.iter().sum();

            // 【保留】约束 A: 整数数值必须精确匹配
            // Sum(x_subset) == TargetInt
            problem.add_constraint(constraint!(sum_expr == target_val));

            // [已移除] 约束 B: Sum(x) - 2*k == TargetBool
        }
    }

    // 7. 求解
    match problem.solve() {
        Ok(solution) => {
            let res = solution.eval(objective);
            let total_count = res.round() as usize;
            Some(total_count)
        }
        Err(e) => {
            eprintln!("❌ 求解失败: {:?}", e);
            None
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
