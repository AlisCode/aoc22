#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(input: &str) -> Choice {
        match input {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => unimplemented!(),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn for_outcome(&self, wanted_outcome: &Outcome) -> Choice {
        match (self, wanted_outcome) {
            (Choice::Rock, Outcome::Win)
            | (Choice::Paper, Outcome::Draw)
            | (Choice::Scissors, Outcome::Lose) => Choice::Paper,
            (Choice::Paper, Outcome::Win)
            | (Choice::Scissors, Outcome::Draw)
            | (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Scissors, Outcome::Win)
            | (Choice::Rock, Outcome::Draw)
            | (Choice::Paper, Outcome::Lose) => Choice::Rock,
        }
    }
}

struct Round {
    pub opponent: Choice,
    pub mine: Choice,
    pub wanted_outcome: Outcome,
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_str(input: &str) -> Self {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unimplemented!(),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (&self.mine, &self.opponent) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(' ');
            let opponent = Choice::from_str(splitted.next().unwrap());
            let second = splitted.next().unwrap();
            let mine = Choice::from_str(second);
            let wanted_outcome = Outcome::from_str(second);
            Round {
                opponent,
                mine,
                wanted_outcome,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Round]) -> u32 {
    input
        .iter()
        .map(|round| round.outcome().value() + round.mine.value())
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Round]) -> u32 {
    input
        .iter()
        .map(|round| {
            let mine = round.opponent.for_outcome(&round.wanted_outcome);
            round.wanted_outcome.value() + mine.value()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn solve_day_2() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 15);
        assert_eq!(part2(&input), 12);
    }
}
