fn main() {
    const INPUT: &str = include_str!("./input.txt");
    println!("part 1: {}", find_visible_trees(&draw_vectors(INPUT)));
    println!("part 1: {}", find_viewing_distance(&draw_vectors(INPUT)));
}

fn draw_vectors(input: &str) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        vec.push(row);
    }
    vec
}

fn find_viewing_distance(input: &[Vec<char>]) -> usize {
    let mut count = Vec::new();

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;

            // Left
            let slice = &row[0..j];
            for tree in slice.iter().rev() {
                left += 1;
                if c <= tree {
                    break;
                }
            }

            // Right
            let slice = &row[j + 1..];
            for tree in slice.iter() {
                right += 1;
                if c <= tree {
                    break;
                }
            }

            // Up
            let slice = &input[0..i];
            let mut v = Vec::new();
            for row in slice {
                v.push(row[j]);
            }

            for tree in v.iter().rev() {
                up += 1;
                if c <= tree {
                    break;
                }
            }

            // Down
            let slice = &input[i + 1..];
            let mut v = Vec::new();

            for row in slice {
                v.push(row[j]);
            }
            for tree in v.iter() {
                down += 1;
                if c <= tree {
                    break;
                }
            }

            count.push(left * right * up * down);
        }
    }

    *count.iter().max().unwrap()
}

fn find_visible_trees(input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 || i == 0 || i == input.len() - 1 {
                count += 1;
            } else {
                let slice = &row[0..=j];
                let max = slice.iter().max().unwrap();
                let occurences = slice.iter().filter(|&n| n == c).count();

                if c == max && occurences == 1 {
                    count += 1;
                    continue;
                }

                let slice = &row[j..];
                let max = slice.iter().max().unwrap();
                let occurences = slice.iter().filter(|&n| n == c).count();

                if c == max && occurences == 1 {
                    count += 1;
                    continue;
                }

                let slice = &input[0..=i];
                let mut v = Vec::new();
                for row in slice {
                    v.push(row[j]);
                }
                let max = v.iter().max().unwrap();
                let occurences = v.iter().filter(|&n| n == c).count();

                if c == max && occurences == 1 {
                    count += 1;
                    continue;
                }

                let slice = &input[i..];
                let mut v = Vec::new();
                for row in slice {
                    v.push(row[j]);
                }
                let max = v.iter().max().unwrap();
                let occurences = v.iter().filter(|&n| n == c).count();

                if c == max && occurences == 1 {
                    count += 1;
                    continue;
                }
            }
        }
    }
    count
}
