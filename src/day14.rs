use aoc_parse::prelude::*;
use pathfinding::prelude::Matrix;

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Air,
    Rock,
    Sand,
}

struct Point {
    pub x: usize,
    pub y: usize,
}

struct ScanResult {
    map: Matrix<BlockType>,
    max_height: usize,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> ScanResult {
    let point = parser!(
        x:usize "," y:usize
        => Point { x, y }
    );
    let structure = parser!(repeat_sep(point, " -> "));
    let structures = parser!(lines(structure));

    let mut map = Matrix::new(700, 200, BlockType::Air);
    let mut max_height = 0;
    let parsed = structures.parse(input).unwrap();
    for l in parsed.into_iter() {
        for (a, b) in l.iter().zip(l.iter().skip(1)) {
            max_height = max_height.max(a.y).max(b.y);
            let min_y = a.y.min(b.y);
            let min_x = a.x.min(b.x);
            if a.x == b.x {
                // Vertical slice
                let max_y = a.y.max(b.y);
                for y in min_y..=max_y {
                    map[(a.x, y)] = BlockType::Rock;
                }
            } else {
                // Horizontal slice
                let max_x = a.x.max(b.x);
                for x in min_x..=max_x {
                    map[(x, a.y)] = BlockType::Rock;
                }
            };
        }
    }
    ScanResult {
        map,
        max_height: max_height + 2,
    }
}

fn spawn_sand(map: &Matrix<BlockType>) -> Option<(usize, usize)> {
    let mut x = 500;
    let mut y = 0;
    loop {
        let below = map.move_in_direction((x, y), (0, 1))?;
        let block = map[below];
        match block {
            BlockType::Air => {
                y += 1;
                continue;
            }
            BlockType::Rock | BlockType::Sand => {
                let left = map.move_in_direction((x, y), (-1, 1))?;
                let block = map[left];
                match block {
                    BlockType::Air => {
                        x -= 1;
                        y += 1;
                        continue;
                    }
                    BlockType::Rock | BlockType::Sand => {
                        let right = map.move_in_direction((x, y), (1, 1))?;
                        let block = map[right];
                        match block {
                            BlockType::Air => {
                                x += 1;
                                y += 1;
                                continue;
                            }
                            BlockType::Rock | BlockType::Sand => {
                                return Some((x, y));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[aoc(day14, part1)]
fn part1(input: &ScanResult) -> usize {
    let mut map = input.map.clone();
    (0..)
        .find(|_| match spawn_sand(&map) {
            Some((x, y)) => {
                map[(x, y)] = BlockType::Sand;
                false
            }
            None => true,
        })
        .unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &ScanResult) -> usize {
    let ScanResult { map, max_height } = input;
    let mut map = map.clone();
    // Add a floor
    for x in 0usize..700usize {
        map[(x, *max_height)] = BlockType::Rock;
    }

    (1..)
        .find(|_| match spawn_sand(&map) {
            Some((x, y)) => {
                if x == 500 && y == 0 {
                    return true;
                }
                map[(x, y)] = BlockType::Sand;
                false
            }
            None => true,
        })
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn solve_day_14() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 24);
        assert_eq!(part2(&input), 93);
    }
}
