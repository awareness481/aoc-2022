use indextree::Arena;
use std::fmt::Display;

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let (part_1, part_2) = parse_input(INPUT);
    println!("part 1: {part_1}\npart 2: {part_2}");
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    amount: i32,
}

impl Display for Directory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.amount)
    }
}

fn parse_input(input: &str) -> (i32, i32) {
    let mut arena = Arena::new();
    let root = arena.new_node(Directory {
        name: "root",
        amount: 0,
    });
    let mut current_directory = root;

    for line in input.lines() {
        let mut parts = line.split(' ');

        match parts.next() {
            Some("$") => {
                let command = parts.next().unwrap();

                match command {
                    "cd" => {
                        let directory = parts.next().unwrap();
                        let mut directory_exists = false;
                        if directory == ".." {
                            current_directory = current_directory.ancestors(&arena).nth(1).unwrap();
                        } else {
                            for child in current_directory.children(&arena) {
                                if arena.get(child).unwrap().get().name == directory {
                                    current_directory = child;
                                    directory_exists = true;
                                    break;
                                }
                            }

                            if !directory_exists {
                                let new_directory = &mut arena.new_node(Directory {
                                    name: directory,
                                    amount: 0,
                                });
                                current_directory.append(*new_directory, &mut arena);
                                current_directory = *new_directory;
                            }
                        }
                    }

                    _ls => {}
                }
            }
            Some("dir") => {
                let directory = parts.next().unwrap();
                let new_directory = arena.new_node(Directory {
                    name: directory,
                    amount: 0,
                });
                current_directory.append(new_directory, &mut arena);
            }
            Some(i) => {
                let amount = i.parse::<i32>().unwrap();
                let cur = arena.get_mut(current_directory).unwrap();
                cur.get_mut().amount += amount;
            }
            _ => {}
        }
    }

    let mut amount = 0;
    let mut vec = Vec::new();

    for child in arena.iter() {
        let mut n = 0;
        let id = arena.get_node_id(child).unwrap();
        for child in id.descendants(&arena) {
            n += arena.get(child).unwrap().get().amount;
        }

        if n <= 100000 {
            amount += n;
        }
        vec.push(Directory {
            name: child.get().name,
            amount: n,
        })
    }

    let total_size = arena.iter().fold(0, |acc, x| acc + x.get().amount);
    let required_free_space = (70000000 - total_size - 30000000).abs();
    let min_size = vec
        .iter()
        .filter(|x| x.amount > required_free_space)
        .map(|x| x.amount)
        .min()
        .unwrap();

    (amount, min_size)
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        const INPUT: &str = include_str!("./test_input.txt");
        let (part_1, part_2) = parse_input(INPUT);
        assert_eq!(part_1, 95437);
        assert_eq!(part_2, 24933642);
    }
}
