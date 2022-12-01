fn main() {
    const INPUT: &str = include_str!("./input.txt");

    let binding = INPUT.split('\n').collect::<Vec<&str>>();
    let mut vec = binding
        .split(|e| e.is_empty())
        .map(|f| {
            let mut sum = 0;
            for i in f.iter() {
                sum += i.parse::<i32>().unwrap();
            }
            sum
        })
        .collect::<Vec<i32>>();

    let max = vec.iter().max().unwrap();
    println!("part 1: {max}");
    vec.sort();
    println!("part 2: {}", vec.iter().rev().take(3).sum::<i32>());
}
