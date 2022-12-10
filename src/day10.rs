use itertools::Itertools;

enum Instruction {
    Noop,
    Addx(i32),
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("addx") {
                let mut input = l.split(" ");
                let _addx = input.next().unwrap();
                let value = input.next().unwrap().parse().unwrap();
                return Instruction::Addx(value);
            }
            return Instruction::Noop;
        })
        .collect()
}

// This does allocate a Vec when we probably don't need it, but it makes part1 and part2
// straightforward to read, so we'll take it
fn run_instructions_to_completion(instructions: &[Instruction]) -> Vec<i32> {
    let mut x = 1;
    let mut signals = Vec::new();

    for i in instructions {
        match i {
            Instruction::Addx(value) => {
                signals.push(x);
                signals.push(x);
                x += value;
            }
            Instruction::Noop => {
                signals.push(x);
            }
        }
    }

    signals
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let signals = run_instructions_to_completion(input);
    signals[19] * 20
        + signals[59] * 60
        + signals[99] * 100
        + signals[139] * 140
        + signals[179] * 180
        + signals[219] * 220
}

fn part2(input: &[Instruction]) -> String {
    let signals = run_instructions_to_completion(input);
    let lines = signals.len() / 40;
    (0..lines)
        .map(|line| {
            (0..40)
                .map(|crt| {
                    let cycle = line * 40 + crt;
                    let x = signals[cycle];
                    let cycle = cycle as i32 % 40;
                    if (cycle - 1..=cycle + 1).contains(&x) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

#[aoc(day10, part2)]
fn part2_formatted(input: &[Instruction]) -> String {
    // Actual logic in fn part2, this is just formatted for terminal.
    // This obliterates performance, but we'll take it
    let result = part2(input).replace(".", " ").replace("#", "▓");
    format!("\n{result}")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const IMAGE: &'static str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn solve_day_10() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 13140);
        assert_eq!(part2(&input), IMAGE);
    }
}
