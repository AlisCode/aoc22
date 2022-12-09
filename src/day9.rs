use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut splitted = l.split(" ");
            let dir = splitted.next().unwrap();
            let steps = splitted.next().unwrap().parse::<i32>().unwrap();
            match dir {
                "U" => Instruction::Up(steps),
                "D" => Instruction::Down(steps),
                "L" => Instruction::Left(steps),
                "R" => Instruction::Right(steps),
                _ => unimplemented!(),
            }
        })
        .collect()
}

struct Head((i32, i32));

impl Head {
    pub fn apply_instruction<F: FnMut((i32, i32))>(&mut self, instr: &Instruction, mut f: F) {
        match instr {
            Instruction::Up(s) => {
                (1..=*s).for_each(|y| f((self.0 .0, self.0 .1 + y)));
                self.0 .1 += s;
            }
            Instruction::Down(s) => {
                (1..=*s).for_each(|y| f((self.0 .0, self.0 .1 - y)));
                self.0 .1 -= s;
            }
            Instruction::Left(s) => {
                (1..=*s).for_each(|x| f((self.0 .0 - x, self.0 .1)));
                self.0 .0 -= s;
            }
            Instruction::Right(s) => {
                (1..=*s).for_each(|x| f((self.0 .0 + x, self.0 .1)));
                self.0 .0 += s;
            }
        }
    }
}

struct Knot((i32, i32));

impl Knot {
    pub fn follow(&mut self, head: (i32, i32)) {
        let dx = head.0 - self.0 .0;
        let dy = head.1 - self.0 .1;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }
        if dx >= 2 {
            self.0 .0 += 1;
            if dy >= 1 {
                self.0 .1 += 1;
            } else if dy <= -1 {
                self.0 .1 -= 1;
            }
        } else if dx <= -2 {
            self.0 .0 -= 1;
            if dy >= 1 {
                self.0 .1 += 1;
            } else if dy <= -1 {
                self.0 .1 -= 1;
            }
        } else if dy >= 2 {
            self.0 .1 += 1;
            if dx >= 1 {
                self.0 .0 += 1;
            } else if dx <= -1 {
                self.0 .0 -= 1;
            }
        } else if dy <= -2 {
            self.0 .1 -= 1;
            if dx >= 1 {
                self.0 .0 += 1;
            } else if dx <= -1 {
                self.0 .0 -= 1;
            }
        }
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut head = Head((0, 0));
    let mut tail = Knot((0, 0));
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::default();
    visited_positions.insert((0, 0));

    for inst in input {
        head.apply_instruction(inst, |head_pos| {
            tail.follow(head_pos);
            visited_positions.insert(tail.0);
        });
    }

    visited_positions.len()
}

#[aoc(day9, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut head = Head((0, 0));
    let mut rope: Vec<Knot> = (0..9).map(|_| Knot((0, 0))).collect();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::default();

    for inst in input {
        head.apply_instruction(inst, |head_pos| {
            rope[0].follow(head_pos);
            for i in 1..9 {
                let pos = rope[i - 1].0;
                rope[i].follow(pos);
            }
            visited_positions.insert(rope[8].0);
        })
    }

    visited_positions.len()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_TWO: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn solve_day_9() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 1);

        let input = parse(INPUT_TWO);
        assert_eq!(part2(&input), 36);
    }

    #[test]
    fn day_9_follow() {
        // .....    .....    .....
        // .TH.. -> .T.H. -> ..TH.
        // .....    .....    .....
        let head = (3, 1);
        let mut tail = Knot((1, 1));
        tail.follow(head);
        assert_eq!(tail.0, (2, 1));

        // ...    ...    ...
        // .T.    .T.    ...
        // .H. -> ... -> .T.
        // ...    .H.    .H.
        // ...    ...    ...
        let head = (1, 1);
        let mut tail = Knot((1, 3));
        tail.follow(head);
        assert_eq!(tail.0, (1, 2));

        // .....    .....    .....
        // .....    ..H..    ..H..
        // ..H.. -> ..... -> ..T..
        // .T...    .T...    .....
        // .....    .....    .....
        let head = (2, 3);
        let mut tail = Knot((1, 1));
        tail.follow(head);
        assert_eq!(tail.0, (2, 2));

        // .....    .....    .....
        // .....    .....    .....
        // ..H.. -> ...H. -> ..TH.
        // .T...    .T...    .....
        // .....    .....    .....
        let head = (3, 2);
        let mut tail = Knot((1, 1));
        tail.follow(head);
        assert_eq!(tail.0, (2, 2));

        // .....    .....    .....
        // .T...    .T...    .....
        // ..H.. -> ..... -> ..T..
        // .....    ..H..    ..H..
        // .....    .....    .....
        let head = (2, 1);
        let mut tail = Knot((1, 3));
        tail.follow(head);
        assert_eq!(tail.0, (2, 2));

        // .....    .....    .....
        // .....    .....    .....
        // ..H.. -> .H... -> .HT..
        // ...T.    ...T.    .....
        // .....    .....    .....
        let head = (1, 2);
        let mut tail = Knot((3, 1));
        tail.follow(head);
        assert_eq!(tail.0, (2, 2));

        // .....    .....    .....
        // .....    ...H.    ...H.
        // ..H.. -> ..... -> ..T..
        // .T...    .T...    .....
        // .....    .....    .....
        let head = (3, 3);
        let mut tail = Knot((1, 1));
        tail.follow(head);
        assert_eq!(tail.0, (2, 2));
    }
}
