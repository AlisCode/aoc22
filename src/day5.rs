#[derive(Debug)]
struct Rearrangement {
    stacks: Stacks,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    size: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<String>>);

impl Stacks {
    pub fn handle_instruction(&mut self, inst: &Instruction) {
        let len = self.0[inst.from - 1].len();
        let drained = self.0[inst.from - 1]
            .drain(len - inst.size..)
            .rev()
            .collect::<Vec<_>>();
        self.0[inst.to - 1].extend(drained)
    }

    pub fn handle_instruction_two(&mut self, inst: &Instruction) {
        let len = self.0[inst.from - 1].len();
        let drained = self.0[inst.from - 1]
            .drain(len - inst.size..)
            .collect::<Vec<_>>();
        self.0[inst.to - 1].extend(drained)
    }

    pub fn scan_top(&self) -> String {
        self.0
            .iter()
            .map(|stack| {
                stack
                    .iter()
                    .rev()
                    .find(|x| x.as_str() != " ")
                    .unwrap()
                    .to_string()
            })
            .collect()
    }
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let input: Vec<&str> = input.split(' ').collect();
        let size = input[1].parse().unwrap();
        let from = input[3].parse().unwrap();
        let to = input[5].parse().unwrap();
        Instruction { size, from, to }
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Rearrangement {
    let mut input = input.split("\n\n");
    let plan = input.next().unwrap();
    let plan: Vec<&str> = plan.lines().collect();
    let length = plan[plan.len() - 1].len() / 4 + 1;
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(length);
    for _ in 0..length {
        stacks.push(Vec::with_capacity(plan.len() - 1));
    }
    for l in plan.iter().rev().skip(1) {
        for stack in 0..length {
            let idx = stack * 4 + 1;
            if let Some(v) = l.get(idx..=idx) {
                if v != " " {
                    stacks[stack].push(v.to_string());
                }
            }
        }
    }

    let instructions = input.next().unwrap();
    let instructions = instructions.lines().map(Instruction::parse).collect();

    Rearrangement {
        stacks: Stacks(stacks),
        instructions,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Rearrangement) -> String {
    let mut stacks = input.stacks.clone();
    for inst in &input.instructions {
        stacks.handle_instruction(inst);
    }
    stacks.scan_top()
}

#[aoc(day5, part2)]
fn part2(input: &Rearrangement) -> String {
    let mut stacks = input.stacks.clone();
    for inst in &input.instructions {
        stacks.handle_instruction_two(inst);
    }
    stacks.scan_top()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // I have a vim extension that trims my end of line on save,
    // which messes up the input. Let's import is instead
    const INPUT: &'static str = include_str!("day5_test.txt");

    #[test]
    fn solve_day_five() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), "CMZ");
        assert_eq!(part2(&input), "MCD");
    }

    #[test]
    fn scan_top() {
        let input = parse(INPUT);
        assert_eq!(input.stacks.scan_top(), "NDP".to_string());
    }
}
