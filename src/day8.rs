use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeMap {
    pub map: HashMap<(i32, i32), i32>,
    pub dimensions: (i32, i32),
}

#[aoc_generator(day8)]
fn parse(input: &str) -> TreeMap {
    let mut dim_x = 0;
    let mut dim_y = 0;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            dim_y += 1;
            dim_x = l.len() as i32;
            l.chars().enumerate().map(move |(x, c)| {
                let height = format!("{c}").parse::<i32>().unwrap();
                ((x as i32, y as i32), height)
            })
        })
        .collect();
    TreeMap {
        map,
        dimensions: (dim_x, dim_y),
    }
}

#[aoc(day8, part1)]
fn part1(input: &TreeMap) -> usize {
    input
        .map
        .iter()
        .filter(|((x, y), height)| {
            *x == 0
                || *y == 0
                || *x == input.dimensions.0 - 1
                || *y == input.dimensions.1 - 1
                || (0..*x).all(|xx| input.map.get(&(xx, *y)).unwrap() < height)
                || (*x + 1..input.dimensions.0).all(|xx| input.map.get(&(xx, *y)).unwrap() < height)
                || (0..*y).all(|yy| input.map.get(&(*x, yy)).unwrap() < height)
                || (*y + 1..input.dimensions.1).all(|yy| input.map.get(&(*x, yy)).unwrap() < height)
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &TreeMap) -> i32 {
    input
        .map
        .iter()
        .map(|((x, y), height)| {
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;
            for xx in (0..*x).rev() {
                // Reverse because this goes from the tree out
                left += 1;
                if input.map.get(&(xx, *y)).unwrap() >= height {
                    break;
                }
            }
            for xx in *x + 1..input.dimensions.0 {
                right += 1;
                if input.map.get(&(xx, *y)).unwrap() >= height {
                    break;
                }
            }
            for yy in (0..*y).rev() {
                // Reverse because this goes from the tree out
                up += 1;
                if input.map.get(&(*x, yy)).unwrap() >= height {
                    break;
                }
            }
            for yy in *y + 1..input.dimensions.1 {
                down += 1;
                if input.map.get(&(*x, yy)).unwrap() >= height {
                    break;
                }
            }
            left * right * up * down
        })
        .max()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn solve_day_8() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 21);
        assert_eq!(part2(&input), 8);
    }
}
