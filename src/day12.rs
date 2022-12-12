struct Heightmap {
    map: Vec<u8>,
    width: i32,
    height: i32,
    start: i32,
    end: i32,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Heightmap {
    let mut width = 0;
    let mut height = 0;
    let mut map = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut idx = 0;
    for l in input.lines() {
        width = l.len();
        height += 1;
        for c in l.chars() {
            let elevation = match c {
                'S' => {
                    start = idx;
                    0
                }
                'E' => {
                    end = idx;
                    25
                }
                x => x as u8 - 'a' as u8,
            };
            map.push(elevation);
            idx += 1;
        }
    }
    Heightmap {
        map,
        width: width as i32,
        height,
        start,
        end,
    }
}

fn solve_for_start(input: &Heightmap, start: i32) -> Option<usize> {
    let start_x = start % input.width;
    let start_y = start / input.width;
    let end_x = input.end % input.width;
    let end_y = input.end / input.width;
    let (path, _) = pathfinding::directed::astar::astar(
        &(start_x, start_y),
        |&(x, y)| {
            let elevation = input.map[(y * input.width + x) as usize];
            let neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&(x, y)| {
                    x >= 0
                        && y >= 0
                        && x < input.width
                        && y < input.height
                        && elevation + 1 >= input.map[(y * input.width + x) as usize]
                })
                .map(|pos| (pos, 1))
                .collect::<Vec<_>>();
            neighbors
        },
        |(x, y)| (end_x - x).abs() * (end_y - y).abs(),
        |&pos| pos == (end_x, end_y),
    )?;
    Some(path.len() - 1)
}

#[aoc(day12, part1)]
fn part1(input: &Heightmap) -> usize {
    solve_for_start(input, input.start).unwrap()
}

#[aoc(day12, part2)]
fn part2(input: &Heightmap) -> usize {
    input
        .map
        .iter()
        .enumerate()
        .filter_map(|(idx, elev)| {
            if *elev == 0 {
                return solve_for_start(input, idx as i32);
            }
            None
        })
        .min()
        .unwrap()
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
