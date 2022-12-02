fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let binding = INPUT.split('\n').collect::<Vec<&str>>();
    let score = binding.iter().fold(0, |acc, x| acc + compare(x));
    let score2 = binding.iter().fold(0, |acc, x| acc + match_condition(x));
    println!("part 1: {score}");
    println!("part 2: {score2}");
}

fn shape(character: &str) -> Shapes {
    match character {
        "A" | "X" => Shapes::Rock,
        "B" | "Y" => Shapes::Paper,
        "C" | "Z" => Shapes::Scissors,
        _ => Shapes::Rock,
    }
}

fn shape_score(character: Shapes) -> i32 {
    match character {
        Shapes::Rock => 1,
        Shapes::Paper => 2,
        Shapes::Scissors => 3,
    }
}

fn compare(x: &&str) -> i32 {
    let shapes = x.split(" ").collect::<Vec<&str>>();
    let first = shape(shapes[0]);
    let second = shape(shapes[1]);
    let score = first.compare(&second);
    score + shape_score(second)
}

fn match_condition(x: &&str) -> i32 {
    let shapes = x.split(" ").collect::<Vec<&str>>();
    let first = shape(shapes[0]);
    let sec = shapes[1];

    let second = match sec {
        "X" => first.win(),
        "Y" => first.get(),
        "Z" => first.lose(),
        _ => first.get(),
    };

    let score = first.compare(&second);

    score + shape_score(second)
}

enum Shapes {
    Rock,
    Paper,
    Scissors,
}

impl Shapes {
    fn compare(&self, other: &Shapes) -> i32 {
        match (self, other) {
            (Shapes::Rock, Shapes::Rock) => 3,
            (Shapes::Rock, Shapes::Paper) => 6,
            (Shapes::Rock, Shapes::Scissors) => 0,
            (Shapes::Paper, Shapes::Rock) => 0,
            (Shapes::Paper, Shapes::Paper) => 3,
            (Shapes::Paper, Shapes::Scissors) => 6,
            (Shapes::Scissors, Shapes::Rock) => 6,
            (Shapes::Scissors, Shapes::Paper) => 0,
            (Shapes::Scissors, Shapes::Scissors) => 3,
        }
    }

    fn get(&self) -> Shapes {
        match self {
            Shapes::Rock => Shapes::Rock,
            Shapes::Paper => Shapes::Paper,
            Shapes::Scissors => Shapes::Scissors,
        }
    }

    fn win(&self) -> Shapes {
        match self {
            Shapes::Rock => Shapes::Scissors,
            Shapes::Paper => Shapes::Rock,
            Shapes::Scissors => Shapes::Paper,
        }
    }

    fn lose(&self) -> Shapes {
        match self {
            Shapes::Rock => Shapes::Paper,
            Shapes::Paper => Shapes::Scissors,
            Shapes::Scissors => Shapes::Rock,
        }
    }
}
