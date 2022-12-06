use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    // let binding = INPUT.split('\n').collect::<Vec<&str>>();
    println!("part 1: {}", detect_first_unique(INPUT));
    println!("part 2: {}", detect_first_unique_2(INPUT));
}

fn detect_first_unique(input: &str) -> usize {
    let inp = input.chars().collect::<Vec<char>>();
    let mut final_index = 0;
    for (i, c) in inp.iter().enumerate() {
        let mut set = HashSet::new();
        set.insert(c);
        set.insert(&inp[i + 1]);
        set.insert(&inp[i + 2]);
        set.insert(&inp[i + 3]);

        if set.len() == 4 {
            println!("{:?} {}", &set, i + 4);
            final_index = i + 4;
            break;
        }
    }

    final_index
}

fn detect_first_unique_2(input: &str) -> usize {
    let inp = input.chars().collect::<Vec<char>>();
    let mut final_index = 0;
    for (i, c) in inp.iter().enumerate() {
        let mut set = HashSet::new();
        set.insert(c);

        for j in 1..14 {
            set.insert(&inp[i + j]);
        }

        if set.len() == 14 {
            println!("{:?} {}", &set, i + 14);
            final_index = i + 14;
            break;
        }
    }

    final_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_first_unique() {
        assert_eq!(detect_first_unique("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(detect_first_unique("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(detect_first_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(detect_first_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_detect_first_unique_2() {
        assert_eq!(detect_first_unique_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(detect_first_unique_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(detect_first_unique_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(
            detect_first_unique_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            detect_first_unique_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}
