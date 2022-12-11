#[derive(Debug)]
enum Value {
    Old,
    Fixed(i32),
}

impl Value {
    fn parse(input: &str) -> Value {
        match input {
            "old" => Value::Old,
            x => Value::Fixed(x.parse().unwrap()),
        }
    }

    fn value(&self, old: i32) -> i32 {
        match self {
            Value::Old => old,
            Value::Fixed(x) => *x,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value),
    Mul(Value),
}

impl Operation {
    fn parse(input: &str, value: Value) -> Operation {
        match input {
            "*" => Operation::Mul(value),
            "+" => Operation::Add(value),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct MonkeyNote {
    starting_items: Vec<i32>,
    operation: Operation,
    test_divisible_by: i32,
    if_true: usize,
    if_false: usize,
}

impl MonkeyNote {
    fn parse(input: &str) -> MonkeyNote {
        let mut lines = input.lines();

        let _idx = lines.next().unwrap();

        let start_line = lines.next().unwrap();
        let starting_items = start_line["  Starting items: ".len()..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let op_line = lines.next().unwrap();
        let mut operation_split = op_line["  Operation: new = old ".len()..].split(" ");
        let op = operation_split.next().unwrap();
        let value = operation_split.next().map(Value::parse).unwrap();
        let operation = Operation::parse(op, value);

        let test_line = lines.next().unwrap();
        let test_divisible_by = test_line["  Test: divisible by ".len()..].parse().unwrap();

        let if_true_line = lines.next().unwrap();
        let if_true = if_true_line["    If true: throw to monkey ".len()..]
            .parse()
            .unwrap();

        let if_false_line = lines.next().unwrap();
        let if_false = if_false_line["    If false: throw to monkey ".len()..]
            .parse()
            .unwrap();

        MonkeyNote {
            starting_items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
        }
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<MonkeyNote> {
    input.split("\n\n").map(MonkeyNote::parse).collect()
}

#[derive(Default, Debug)]
struct Monkey {
    worry_levels: Vec<i32>,
    total_items: usize,
}

#[aoc(day11, part1)]
fn part1(notes: &[MonkeyNote]) -> usize {
    let mut monkeys: Vec<Monkey> = notes
        .iter()
        .map(|note| Monkey {
            worry_levels: note.starting_items.clone(),
            total_items: 0,
        })
        .collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::default();
            std::mem::swap(&mut monkey, &mut monkeys[i]);
            monkey.total_items += monkey.worry_levels.len();
            for item in monkey.worry_levels.drain(..) {
                let worry_level = match &notes[i].operation {
                    Operation::Add(v) => item + v.value(item),
                    Operation::Mul(v) => item * v.value(item),
                } / 3;
                let target = if worry_level % notes[i].test_divisible_by == 0 {
                    notes[i].if_true
                } else {
                    notes[i].if_false
                };
                monkeys[target].worry_levels.push(worry_level);
            }
            std::mem::swap(&mut monkey, &mut monkeys[i]);
        }
    }

    monkeys.sort_by(|a, b| b.total_items.cmp(&a.total_items));
    monkeys.iter().take(2).map(|m| m.total_items).product()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn solve_day_11() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 10605);
        //assert!(false);
    }
}
