use std::{collections::HashMap, fs, ops::Index, os::macos::raw};

fn main() {
    // println!("{}", part1("input.txt").unwrap());
    println!("{}", part2("input.txt").unwrap())
}

fn part1(filename: &str) -> Result<u64, String> {
    let problems = parse_input_part1(filename)?;
    Ok(compute_total(problems))
}

fn part2(filename: &str) -> Result<u64, String> {
    let problems = parse_input_part2(filename)?;
    Ok(compute_total(problems))
}

fn compute_total(problems: MathProblems) -> u64 {
    problems.problems.into_iter().fold(0, |acc, p| {
        println!("{:?}", p);
        let acc_val = match p.arithmetic {
            Arithmetic::Add => 0,
            Arithmetic::Multiply => 1,
        };
        let problem_output = p
            .numbers
            .into_iter()
            .fold(acc_val, |acc, num| match p.arithmetic {
                Arithmetic::Add => acc + num,
                Arithmetic::Multiply => acc * num,
            });
        println!("{}", problem_output);
        acc + problem_output
    })
}

#[derive(Debug)]
struct MathProblems {
    problems: Vec<Problem>,
}

#[derive(Debug, Default)]
enum Arithmetic {
    #[default]
    Add,
    Multiply,
}

#[derive(Debug, Default)]
struct Problem {
    numbers: Vec<u64>,
    arithmetic: Arithmetic,
}

fn parse_input_part1(filename: &str) -> Result<MathProblems, String> {
    let raw_file =
        fs::read_to_string(filename).map_err(|e| format!("failed to read file {:?}", e))?;
    let lines = raw_file.lines();
    let mut problems: Vec<Problem> = Vec::new();

    for line in lines {
        for (idx, number) in line.split_whitespace().enumerate() {
            if idx >= problems.len() {
                problems.resize_with(idx + 1, Problem::default);
            }
            let cur_problem = &mut problems[idx];
            if let Ok(parsed_num) = number.parse::<u64>() {
                cur_problem.numbers.push(parsed_num);
            } else {
                let arithmetic = match number {
                    "+" => Arithmetic::Add,
                    "*" => Arithmetic::Multiply,
                    _ => return Err(format!("bad parsing: {}", number)),
                };
                cur_problem.arithmetic = arithmetic;
            }
        }
    }
    Ok(MathProblems { problems })
}

fn parse_input_part2(filename: &str) -> Result<MathProblems, String> {
    let raw_file =
        fs::read_to_string(filename).map_err(|e| format!("failed to read file {:?}", e))?;
    let mut problems: Vec<Problem> = Vec::new();
    let lines: Vec<&str> = raw_file.lines().into_iter().collect();
    // the operator marks the start of each column. Find those, then we can work off that
    let last_line = match lines.last() {
        Some(l) => l,
        None => return Err("bad parse - no lines".to_string()),
    };

    let mut each_start_idx = Vec::new();
    for (i, c) in last_line.chars().enumerate() {
        let operator = match c {
            '+' => Arithmetic::Add,
            '*' => Arithmetic::Multiply,
            _ => continue,
        };
        problems.push(Problem {
            numbers: vec![],
            arithmetic: operator,
        });
        each_start_idx.push(i);
    }

    let mut cols_to_nums: HashMap<usize, String> = HashMap::new();
    for row in lines.into_iter() {
        for col in (0..row.len()).rev() {
            let number = row.chars().nth(col).unwrap();
            if number.is_numeric() {
                if let Some(nums) = cols_to_nums.get_mut(&col) {
                    nums.push(number);
                } else {
                    cols_to_nums.insert(col, number.to_string());
                }
            }
        }
    }


    // This is nasty.. but we're here...
    let mut idx_tracker = 0;
    let mut each_start_idx = each_start_idx.into_iter().skip(1).peekable();
    let mut on_final = false;
    // Iterate over each problem. For each problem, we will loop infinitely until we've hit the next problem's index
    // which is tracked using the idx_tracker and the each_start_idx.peek() below. 
    // The on_final is kind of a result of catching edge cases. There is certainly a better way to do this, but here we are... 
    // Once the each_start_idx has been drained, then on_final is set to true indicating we are on the final problem. 
    // Once that is the case, then the None check on the first match is the ticket out of this infinite loop
    for cur_problem in problems.iter_mut() {
        loop {
            match cols_to_nums.get(&idx_tracker) {
                Some(n) => {
                    cur_problem.numbers.push(n.parse::<u64>().unwrap());
                    println!("Current: {:?} -> {}", cur_problem, n);
                }
                None => {
                    if on_final {
                        break;
                    }
                }
            };
            idx_tracker += 1;

            // If we hit the next index, break out of the while loop and move to the next problem
            match each_start_idx.peek() {
                Some(next_idx) => {
                    if next_idx == &idx_tracker {
                        each_start_idx.next();
                        break;
                    }
                }
                None => {
                    on_final = true;
                }
            }
        }
    }

    Ok(MathProblems { problems })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_part1() {
        assert_eq!(part1("demo_input.txt").unwrap(), 4277556)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("input.txt").unwrap(), 4693419406682)
    }

    #[test]
    fn test_demo_part2() {
        assert_eq!(part2("demo_input.txt").unwrap(), 3263827)
    }
}
