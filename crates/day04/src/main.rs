fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let binding = INPUT.split('\n').collect::<Vec<&str>>();
    println!("part 1: {}", calculate_part1_score(binding.clone()));
    println!("part 2: {}", calculate_part2_score(binding.clone()));
}

fn split_pairs(s: &str) -> ((i32, i32), (i32, i32)) {
    let pairs = s.split(',').collect::<Vec<&str>>();
    let range1 = pairs[0].split('-').collect::<Vec<&str>>();
    let range2 = pairs[1].split('-').collect::<Vec<&str>>();

    (
        (
            range1[0].parse::<i32>().unwrap(),
            range1[1].parse::<i32>().unwrap(),
        ),
        (
            range2[0].parse::<i32>().unwrap(),
            range2[1].parse::<i32>().unwrap(),
        ),
    )
}

fn compare_ranges(range1: (i32, i32), range2: (i32, i32)) -> i32 {
    let r1 = range1.0..=range1.1;
    let r2 = range2.0..=range2.1;

    if (r1).contains(r2.start()) && (r1).contains(r2.end()) {
        return 1;
    }

    if (r2).contains(r1.start()) && (r2).contains(r1.end()) {
        return 1;
    }

    0
}

fn compare_ranges_overlap(range1: (i32, i32), range2: (i32, i32)) -> i32 {
    let r1 = range1.0..=range1.1;
    let r2 = range2.0..=range2.1;

    if (r1).contains(r2.start()) || (r1).contains(r2.end()) {
        return 1;
    }

    if (r2).contains(r1.start()) || (r2).contains(r1.end()) {
        return 1;
    }

    0
}

fn calculate_part1_score(input: Vec<&str>) -> i32 {
    let mut score = 0;
    input.iter().for_each(|line| {
        let (range1, range2) = split_pairs(line);
        score += compare_ranges(range1, range2);
    });
    score
}

fn calculate_part2_score(input: Vec<&str>) -> i32 {
    let mut score = 0;
    input.iter().for_each(|line| {
        let (range1, range2) = split_pairs(line);
        score += compare_ranges_overlap(range1, range2);
    });
    score
}
