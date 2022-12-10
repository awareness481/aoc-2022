fn main() {
    const INPUT: &str = include_str!("./input.txt");
    let (part1, part2) = parse_input(INPUT);
    println!("part 1: {part1}\npart 2: \n{part2}");
}

fn parse_input(input: &str) -> (i32, String) {
    let adapters = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    let mut total = 1;
    let mut cycles = 0;
    let mut part1 = 0;
    let mut part2 = String::with_capacity(40 * 6);

    for adapter in adapters.iter() {
        let instruction = adapter[0];
        let mut inc = 0;
        let mut current_cycles = 0;

        if instruction == "addx" {
            let value = adapter[1].parse::<i32>().unwrap();
            inc += value;
            current_cycles = 2;
        } else {
            current_cycles = 1;
        }

        for _ in 0..current_cycles {
            let mut pixel = '.';
            if (cycles % 40_i32 - total).abs() <= 1 {
                pixel = '#';
            }

            cycles += 1;
            if cycles == 20
                || cycles == 60
                || cycles == 100
                || cycles == 140
                || cycles == 180
                || cycles == 220
            {
                part1 += total * cycles;
            }

            part2.push(pixel);
            if cycles % 40 == 0 {
                part2.push('\n');
            }
        }

        total += inc;
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("./test_input.txt")
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(input()).0, 13140);
        assert_eq!(
            parse_input(input()).1,
            "##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n"
                .to_string()
        )
    }
}
