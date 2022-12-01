#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i32> {
    let mut calories = Vec::new();
    let mut buffer = 0;
    for i in input.lines() {
        if i == "" {
            calories.push(buffer);
            buffer = 0;
            continue;
        }
        buffer += i.parse::<i32>().unwrap();
    }
    calories.push(buffer);
    calories.sort_by(|a, b| b.cmp(a));
    calories
}

#[aoc(day1, part1)]
fn part1(calories: &[i32]) -> i32 {
    calories[0]
}

#[aoc(day1, part2)]
fn part2(calories: &[i32]) -> i32 {
    calories.iter().take(3).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn solve_day_1() {
        let input = parse(INPUT);
        dbg!(&input);
        assert_eq!(part1(&input), 24000);
        assert_eq!(part2(&input), 45000);
    }
}
