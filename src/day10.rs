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
    let machines = input
        .lines()
        .filter_map(|line| Machine::from_str(line))
        .collect::<Vec<Machine>>();

    let res = machines
        .iter()
        .map(|machine| {
            println!(
                "Checking machine with target lights: {:?}",
                machine.target_light
            );
            let mut min_count = usize::MAX;
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
                    let added_joltage: Vec<i32> = perm
                        .iter()
                        .map(|&btn_idx| machine.button[btn_idx].clone())
                        .fold(vec![0; machine.joltage.len()], |mut acc, button| {
                            for &idx in &button {
                                acc[idx] += 1;
                            }
                            acc
                        });
                    println!("Added joltage from buttons: {:?}", added_joltage);
                    let remain_joltage: Vec<i32> = machine
                        .joltage
                        .iter()
                        .enumerate()
                        .map(|(i, &joltage)| joltage as i32 - added_joltage[i])
                        .collect();
                    println!("Remaining joltage to fulfill: {:?}", remain_joltage);
                    if let Some(count) = solve(remain_joltage, &machine.target_light, &machine.button) {
                        if count + perm.len() < min_count {
                            min_count = count + perm.len();
                        }
                    }
                }
            });
            min_count
        })
        .sum::<usize>();
    Ok(res.to_string())
}

fn solve(
    target_ints: Vec<i32>,
    target_bools: &Vec<bool>,
    subsets: &Vec<Vec<usize>>,
) -> Option<usize> {
    // 1. 初始化变量构建器
    let mut vars = variables!();

    // 2. 创建主要决策变量 x_i (每个子集取多少个)
    let x_vars: Vec<_> = (0..subsets.len())
        .map(|i| vars.add(variable().min(0).integer().name(format!("x{}", i))))
        .collect();

    // 3. 创建辅助变量 k_i (用于处理布尔值的模 2 约束)
    // 对于每个目标位置 i，我们需要一个 k_i 来表示翻转次数里的 "偶数部分"
    // 公式: Total_Flips = 2 * k + Target_Bool
    let k_vars: Vec<_> = (0..target_ints.len())
        .map(|i| vars.add(variable().min(0).integer().name(format!("k_aux_{}", i))))
        .collect();

    // 4. 目标函数: 最小化 x 的总和
    let objective: Expression = x_vars.iter().sum();

    // 5. 初始化模型
    let mut problem = vars.minimise(objective.clone()).using(highs);

    // 6. 生成约束
    for pos in 0..target_ints.len() {
        let target_val = target_ints[pos];
        let target_bool_val = if target_bools[pos] { 1 } else { 0 };

        // --- 找出覆盖当前 pos 的 x 变量 ---
        // (在这个题目中，子集既影响 int 也影响 bool，所以是同一组 x)
        let relevant_vars: Vec<Expression> = subsets
            .iter()
            .enumerate()
            .filter(|(_, subset)| subset.contains(&pos))
            .map(|(i, _)| x_vars[i].into())
            .collect();

        if relevant_vars.is_empty() {
            // 没有任何子集能影响这个位置
            if target_val > 0 || target_bool_val > 0 {
                eprintln!("❌ 位置 {} 无法被覆盖 (无相关子集)", pos);
                return None;
            }
        } else {
            let sum_expr: Expression = relevant_vars.iter().sum();

            // 【约束 A: 整数数值必须精确匹配】
            // Sum(x) == TargetInt
            problem.add_constraint(constraint!(sum_expr.clone() == target_val));

            // 【约束 B: 布尔值必须匹配 (模 2 约束)】
            // Sum(x) == 2 * k + TargetBool
            // 移项得: Sum(x) - 2 * k == TargetBool
            let bool_constraint_expr = sum_expr - 2 * k_vars[pos];
            problem.add_constraint(constraint!(bool_constraint_expr == target_bool_val));
        }
    }

    // 7. 求解
    println!("正在求解 (双重约束: 数值 + 布尔)...");
    match problem.solve() {
        Ok(solution) => {
            let res = solution.eval(objective);
            let total_count = res.round() as usize;

            println!("✅ 找到最优解! 最小总数: {}", total_count);
            println!("{}", "-".repeat(40));

            // 打印 x 的取值
            for (i, subset) in subsets.iter().enumerate() {
                let count = solution.value(x_vars[i]);
                if count > 0.0 {
                    println!(
                        "  集合 {:<10}: 取 {} 个",
                        format!("{:?}", subset),
                        count.round() as i32
                    );
                }
            }

            // (可选) 打印 k 的取值，用于验证布尔逻辑
            // k 表示在这个位置上，我们翻转了多少个 "双倍"
            /*
            println!("{}", "-".repeat(40));
            for pos in 0..target_ints.len() {
                 let k = solution.value(k_vars[pos]);
                 println!("  Pos {}: 辅助变量 k={}, 总翻转次数={}",
                    pos, k.round(), 2.0 * k + (if target_bools[pos] {1.0} else {0.0}));
            }
            */

            Some(total_count)
        }
        Err(_) => {
            // 为了更友好的提示，我们不打印复杂的 error struct，而是说明原因
            eprintln!("❌ 求解失败: 无法同时满足【数值目标】和【布尔目标】。");
            eprintln!(
                "   原因可能是数学冲突 (例如: 要求数值凑出 3(奇)，但布尔状态要求 False(偶))。"
            );
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
    #[test]
    fn test_highs() {
        let subsets = vec![
            vec![3],    // x0
            vec![1, 3], // x1
            vec![2],    // x2
            vec![2, 3], // x3
            vec![0, 2], // x4
            vec![0, 1], // x5
        ];

        let target_ints = vec![3, 5, 4, 7];

        // 这里的 bool 必须符合 target_ints 的奇偶性，否则会报错
        // 3->T, 5->T, 4->F, 7->T
        let target_bools = vec![true, true, false, true];

        // 测试一个不可能的案例 (解开注释试试)
        // let target_bools = vec![true, true, true, true]; // 错误: 位置2是4(偶)，不能是True

        solve(target_ints, target_bools, &subsets);
    }
}
