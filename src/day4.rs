use aoc_parse::prelude::*;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Assignment {
    one: RangeInclusive<usize>,
    two: RangeInclusive<usize>,
}

fn range_includes(r: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r.start() <= r2.start() && r.end() >= r2.end()
}

fn range_overlaps(r: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r2.contains(r.start()) || r2.contains(r.end())
}

#[aoc_generator(day4)]
fn parse(input: &str) -> anyhow::Result<Vec<Assignment>> {
    aoc_parse(
        input,
        parser!(
            lines(
                a:usize "-" b:usize "," c:usize "-" d:usize
                => Assignment { one: (a..=b), two: (c..=d) }
            )
        ),
    )
}

#[aoc(day4, part1)]
fn part1(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|a| range_includes(&a.one, &a.two) || range_includes(&a.two, &a.one))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|a| range_overlaps(&a.one, &a.two) || range_overlaps(&a.two, &a.one))
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn solve_day_4() {
        let input = parse(INPUT).unwrap();
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
