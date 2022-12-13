use pathfinding::prelude::{bfs, Matrix};

struct Heightmap {
    map: Matrix<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Heightmap {
    let mut map = Matrix::from_rows(input.lines().map(str::bytes)).expect("Failed to build map");
    // I'm very sad we can't find those in one pass (when building the Matrix),
    // but I couldn't find how!
    let start = map.indices().find(|p| map[*p] == b'S').unwrap();
    let end = map.indices().find(|p| map[*p] == b'E').unwrap();
    map[start] = b'a';
    map[end] = b'z';
    Heightmap { map, start, end }
}

#[aoc(day12, part1)]
fn part1(input: &Heightmap) -> usize {
    let path = bfs(
        &input.start,
        |&p| {
            input
                .map
                .neighbours(p, false)
                .filter(move |n| input.map[p] + 1 >= input.map[*n])
        },
        |&p| p == input.end,
    )
    .expect("Failed to find path");
    path.len() - 1
}

#[aoc(day12, part2)]
fn part2(input: &Heightmap) -> usize {
    let path = bfs(
        &input.end,
        |&p| {
            input
                .map
                .neighbours(p, false)
                .filter(move |n| input.map[p] <= input.map[*n] + 1)
        },
        |&p| input.map[p] == b'a',
    )
    .expect("Failed to find path");
    path.len() - 1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn solve_day_12() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }
}
