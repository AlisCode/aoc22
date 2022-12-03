use std::collections::HashSet;

struct Rucksack {
    one: String,
    two: String,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| {
            let compartment_size = l.len() / 2;
            Rucksack {
                one: l[0..compartment_size].to_string(),
                two: l[compartment_size..].to_string(),
            }
        })
        .collect()
}

fn item_priority(c: char) -> u32 {
    if c.is_uppercase() {
        27 + c as u32 - 'A' as u32
    } else {
        1 + c as u32 - 'a' as u32
    }
}

#[aoc(day3, part1)]
fn part1(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .map(|sack| {
            let one = sack.one.chars().collect::<HashSet<char>>();
            let two = sack.two.chars().collect::<HashSet<char>>();
            item_priority((&one & &two).into_iter().next().unwrap())
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Rucksack]) -> u32 {
    input
        .chunks(3)
        .map(|sacks| {
            let one = sacks[0]
                .one
                .chars()
                .chain(sacks[0].two.chars())
                .collect::<HashSet<char>>();
            let two = sacks[1]
                .one
                .chars()
                .chain(sacks[1].two.chars())
                .collect::<HashSet<char>>();
            let three = sacks[2]
                .one
                .chars()
                .chain(sacks[2].two.chars())
                .collect::<HashSet<char>>();
            let inter = &(&one & &two) & &three;
            item_priority(inter.into_iter().next().unwrap())
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn solve_day_three() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 157);
        assert_eq!(part2(&input), 70);
    }
}
