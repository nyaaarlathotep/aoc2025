use std::env;
use std::fs;
use std::process;
use std::time::Instant;

// ==========================================
// 在这里注册你的天数模块
// 每做新的一天，就在这里加上 dayXX
// ==========================================
aoc_main!(
    day01
    // day01,
    // day02,
);

// 下面是宏定义，不需要每次修改
#[macro_export]
macro_rules! aoc_main {
    ($($day:ident),*) => {
        // 1. 自动生成 mod 声明
        $(pub mod $day;)*

        fn main() {
            // 获取命令行参数
            let args: Vec<String> = env::args().collect();
            
            // 如果没有传参数，默认运行列表里的最后一个（最新的一天）
            let day_arg = if args.len() > 1 {
                args[1].as_str()
            } else {
                // 获取最新的模块名作为默认值 (取最后一个 ident)
                let days = vec![$(stringify!($day)),*];
                *days.last().unwrap()
            };

            // 处理输入格式：如果是 "25"，自动拼成 "day25" 以便匹配
            let day_module_name = if day_arg.starts_with("day") {
                day_arg.to_string()
            } else {
                format!("day{}", day_arg)
            };

            println!("Running AoC: [{}]", day_module_name);

            // 2. 自动生成 match 分发逻辑
            match day_module_name.as_str() {
                $(
                    stringify!($day) => {
                        run_day(stringify!($day), |input| {
                            println!("--- Part 1 ---");
                            println!("Result: {:?}", $day::part01(input));
                            println!("--- Part 2 ---");
                            println!("Result: {:?}", $day::part02(input));
                        });
                    }
                )*
                _ => {
                    eprintln!("Error: Module '{}' not found in registration list.", day_module_name);
                    process::exit(1);
                }
            }
        }
    };
}

// 统一的运行逻辑
fn run_day<F>(day_name: &str, solve_fn: F)
where
    F: Fn(&str),
{
    // 构造路径，例如 ./src/day25/input
    let input_path = format!("./inputs/{}.txt", day_name);
    
    match read_file_string(&input_path) {
        Ok(input) => {
            let start = Instant::now();
            solve_fn(&input);
            let duration = start.elapsed();
            println!("----------------");
            println!("Time elapsed: {:?}", duration);
        }
        Err(e) => {
            eprintln!("Failed to read input file '{}': {}", input_path, e);
        }
    }
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
