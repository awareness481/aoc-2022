use regex::Regex;
use std::{collections::VecDeque, vec};

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let binding = INPUT.split('\n').collect::<Vec<&str>>();
    println!("part 1: {}", build_stack(binding.clone()));
    println!("part 1: {}", build_stack_part2(binding.clone()));
}

fn build_stack(input: Vec<&str>) -> String {
    let mut stack: Vec<VecDeque<&str>> = Vec::new();
    for line in input {
        if line.starts_with('[') || line.contains('[') {
            let vec = parse_line(line);

            vec.iter().enumerate().for_each(|(index, x)| {
                if stack.len() <= index {
                    stack.push(VecDeque::new());
                }
                stack[index].push_back(x);
            });
        } else if let Some(inst) = parse_instruction(line) {
            let (move_t, from, to) = inst;

            for _ in 0..move_t {
                for (i, el) in stack[from as usize].clone().into_iter().enumerate() {
                    if !el.is_empty() {
                        stack[from as usize].remove(i);
                        stack[to as usize].push_front(el);
                        break;
                    }
                }
            }
        }
    }

    let mut st = String::new();
    for i in stack.clone().into_iter() {
        if !i.is_empty() {
            st.push_str(i.front().unwrap()[1..2].to_string().as_str());
        }
    }

    st
}

fn build_stack_part2(input: Vec<&str>) -> String {
    let mut stack: Vec<VecDeque<&str>> = Vec::new();
    for line in input {
        if line.starts_with('[') || line.contains('[') {
            let vec = parse_line(line);

            vec.iter().enumerate().for_each(|(index, x)| {
                if stack.len() <= index {
                    stack.push(VecDeque::new());
                }
                stack[index].push_back(x);
            });
        } else if let Some(inst) = parse_instruction(line) {
            let (move_t, from, to) = inst;
            let mut vector = Vec::new();
            for _ in 0..move_t {
                for (i, el) in stack[from as usize].clone().into_iter().enumerate() {
                    if !el.is_empty() {
                        let el = stack[from as usize].remove(i).unwrap();
                        vector.push(el);
                        break;
                    }
                }
            }

            for el in vector.into_iter().rev() {
                stack[to as usize].push_front(el);
            }
        }
    }

    let mut st = String::new();
    for i in stack.clone().into_iter() {
        if !i.is_empty() {
            st.push_str(i.front().unwrap()[1..2].to_string().as_str());
        }
    }
    st
}

fn parse_line(line: &str) -> Vec<&str> {
    let mut stack: Vec<&str> = vec![];
    let vec = line.split(' ').collect::<Vec<&str>>();

    let mut count = 0;

    for c in vec.iter() {
        if c.is_empty() {
            if count == 3 {
                count = 0;
                stack.push("");
            } else {
                count += 1;
            }
        } else {
            stack.push(c);
        }
    }

    stack
}

fn parse_instruction(text: &str) -> Option<(i8, i8, i8)> {
    let re = Regex::new(r"(?m)((move)(?: )(\d+)|(from)(?: )(\d+)|(to)(?: )(\d+))");
    let mut instruction: Vec<i8> = Vec::new();
    for cap in re.unwrap().captures_iter(text) {
        let index = &cap[0].split(' ').collect::<Vec<&str>>()[1];
        instruction.push(index.parse::<i8>().unwrap());
    }

    if instruction.is_empty() {
        return None;
    }

    Some((instruction[0], instruction[1] - 1, instruction[2] - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "[P]     [C]         [M]            ";
        let result = parse_line(line);
        assert_eq!(result, vec!["[P]", "", "[C]", "", "", "[M]", "", "", ""]);
    }
}
