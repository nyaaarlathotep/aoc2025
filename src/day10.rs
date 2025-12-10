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
    Ok("solved part 2".to_string())
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
        assert_eq!(part02(&INPUT).unwrap(), "6");
    }
}
