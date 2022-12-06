use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    // let binding = INPUT.split('\n').collect::<Vec<&str>>();
    println!("part 1: {}", detect_first_unique(INPUT, 4));
    println!("part 2: {}", detect_first_unique(INPUT, 14));
}

fn detect_first_unique(input: &str, size: usize) -> usize {
    let binding = input.chars().into_iter().collect::<Vec<char>>();
    let windows = binding.windows(size);

    let mut final_index = 0;

    for (i, window) in windows.enumerate() {
        let mut set = HashSet::new();

        for c in window {
            set.insert(c);
        }

        if set.len() == size {
            final_index = i + size;
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
        assert_eq!(detect_first_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(detect_first_unique("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            detect_first_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            detect_first_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }

    #[test]
    fn test_detect_first_unique_2() {
        assert_eq!(
            detect_first_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(detect_first_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(detect_first_unique("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            detect_first_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            detect_first_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
