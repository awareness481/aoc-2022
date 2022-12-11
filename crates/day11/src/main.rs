use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let (p1, p2) = parse_input(INPUT, 8);
    println!("part 1: {p1}\npart 2: {p2}");
}

#[derive(Default, Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    divide_by: u128,
    true_condition: i32,
    false_condition: i32,
}

#[derive(Default, Clone, Debug)]
enum Operation {
    Add(Option<u128>),
    Multiply(Option<u128>),
    #[default]
    None,
}

impl Monkey {
    fn new() -> Self {
        Default::default()
    }
}

fn parse_input(input: &str, size: usize) -> (u128, u128) {
    let inputs = input.lines().collect::<Vec<&str>>();
    let mut monkeys: Vec<u128> = vec![0; size];
    let mut m: Vec<Monkey> = vec![Monkey::new(); size];

    for (monkey_n, input) in inputs.chunks(7).enumerate() {
        for (i, line) in input.iter().enumerate() {
            match i {
                1 => {
                    let (_, text) = line.split_at(line.find(':').unwrap());

                    let items = text[1..]
                        .trim()
                        .split(',')
                        .filter(|x| !x.is_empty())
                        .map(|x| x.trim().parse::<u128>().unwrap())
                        .collect::<Vec<u128>>();
                    m[monkey_n].items = items;
                }
                2 => {
                    let operation_re = Regex::new(r"(?:Operation: new = old )(.+)");
                    let op = operation_re.unwrap().captures(line).unwrap();
                    let (symbol, number) = op[1].split_at(1);

                    match symbol {
                        "+" => {
                            let number = number.trim();
                            if number == "old" {
                                m[monkey_n].operation = Operation::Add(None);
                            } else {
                                m[monkey_n].operation =
                                    Operation::Add(Some(number.parse::<u128>().unwrap()));
                            }
                        }
                        "*" => {
                            let number = number.trim();
                            if number == "old" {
                                m[monkey_n].operation = Operation::Multiply(None);
                            } else {
                                m[monkey_n].operation =
                                    Operation::Multiply(Some(number.parse::<u128>().unwrap()));
                            }
                        }
                        _ => {}
                    }
                }
                3 => {
                    let divisible_re = Regex::new(r"(?:Test: divisible by )(.+)");
                    let divisible = divisible_re.unwrap().captures(line).unwrap();
                    m[monkey_n].divide_by = divisible[1].parse::<u128>().unwrap();
                }
                4 => {
                    let true_re = Regex::new(r"(?:If true: throw to monkey )(.+)");
                    let true_number = true_re.unwrap().captures(line).unwrap();
                    m[monkey_n].true_condition = true_number[1].parse::<i32>().unwrap();
                }
                5 => {
                    let false_re = Regex::new(r"(?:If false: throw to monkey )(.+)");
                    let false_number = false_re.unwrap().captures(line).unwrap();
                    m[monkey_n].false_condition = false_number[1].parse::<i32>().unwrap();
                }
                _ => {}
            }
        }
    }

    let mut d = m.clone();

    for _ in 0..20 {
        for x in 0..d.len() {
            for item in d[x].clone().items.iter() {
                monkeys[x] += 1;

                let mut level: u128 = match d[x].operation {
                    Operation::Add(Some(num)) => *item + num,
                    Operation::Add(None) => *item * 2,
                    Operation::Multiply(Some(num)) => *item * num,
                    Operation::Multiply(None) => *item * *item,
                    Operation::None => 0,
                };
                level /= 3;

                if level % d[x].divide_by == 0 {
                    let y = d[x].true_condition as usize;

                    let mut items = d[y].items.clone();
                    items.push(level);
                    d[y].items = items.to_vec();
                } else {
                    let y = d[x].false_condition as usize;

                    let mut items = d[y].items.clone();
                    items.push(level);
                    d[y].items = items.to_vec();
                }
            }
            d[x].items = Vec::new();
        }
    }

    monkeys.sort();
    let p1_result = monkeys.iter().rev().take(2).product::<u128>();
    monkeys = vec![0; 8];

    let mut d = m.clone();
    let modulus = m.iter().fold(1, |acc, x| acc * x.divide_by);
    for _ in 0..10000 {
        for x in 0..d.len() {
            for item in d[x].clone().items.iter() {
                monkeys[x] += 1;

                let level: u128 = match d[x].operation {
                    Operation::Add(Some(num)) => *item + num,
                    Operation::Add(None) => *item * 2,
                    Operation::Multiply(Some(num)) => *item * num,
                    Operation::Multiply(None) => *item * *item,
                    Operation::None => 0,
                };

                if level % d[x].divide_by == 0 {
                    let y = d[x].true_condition as usize;

                    let mut items = d[y].items.clone();
                    items.push(level % modulus);
                    d[y].items = items.to_vec();
                } else {
                    let y = d[x].false_condition as usize;

                    let mut items = d[y].items.clone();
                    items.push(level % modulus);
                    d[y].items = items.to_vec();
                }
            }
            d[x].items = Vec::new();
        }
    }

    monkeys.sort();
    let p2_result = monkeys.iter().rev().take(2).product::<u128>();
    (p1_result, p2_result)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        const INPUT: &str = include_str!("./test_input.txt");
        assert_eq!(parse_input(INPUT, 4).0, 10605);
        assert_eq!(parse_input(INPUT, 4).1, 2713310158);
    }
}
