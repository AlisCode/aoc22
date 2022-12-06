use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, window_size: usize) -> usize {
    let input: Vec<char> = input.chars().collect();
    let (idx, _) = input
        .windows(window_size)
        .enumerate()
        .find(|(_idx, w)| {
            let set = w.iter().collect::<HashSet<_>>();
            set.len() == window_size
        })
        .unwrap();
    idx + window_size
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn solve_day_6() {
        // part 1
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        // part2
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
