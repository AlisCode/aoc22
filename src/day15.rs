use std::ops::RangeInclusive;

use aoc_parse::prelude::*;
use itertools::Itertools;

#[derive(Debug)]
struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
struct BeaconInfo {
    pub sensor: Point,
    pub beacon: Point,
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<BeaconInfo> {
    let info = parser!(
        "Sensor at x=" sensor_x:isize ", y=" sensor_y:isize ": closest beacon is at x=" beacon_x:isize ", y=" beacon_y:isize
        => BeaconInfo { sensor: Point { x: sensor_x, y: sensor_y }, beacon: Point { x: beacon_x, y: beacon_y } }
    );
    parser!(lines(info)).parse(input).unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &[BeaconInfo]) -> isize {
    impossible_positions_for_axis(input, Axis::Row(2000000), None)
}

#[aoc(day15, part2)]
fn part2(input: &[BeaconInfo]) -> isize {
    find_beacon_frequency(input, 4000000)
}

fn coalesce_ranges(
    a: RangeInclusive<isize>,
    b: RangeInclusive<isize>,
) -> Result<RangeInclusive<isize>, (RangeInclusive<isize>, RangeInclusive<isize>)> {
    let min_start = *a.start().min(b.start());
    let min_end = *a.end().min(b.end());
    let max_start = *a.start().max(b.start());
    let max_end = *a.end().max(b.end());
    if max_start > min_end {
        return Err((min_start..=min_end, max_start..=max_end));
    }
    Ok(min_start..=max_end)
}

enum Axis {
    Row(isize),
    Col(isize),
}

fn impossible_positions_for_axis(input: &[BeaconInfo], axis: Axis, limit: Option<isize>) -> isize {
    input
        .iter()
        .filter_map(|info| {
            let dx = (info.sensor.x - info.beacon.x).abs();
            let dy = (info.sensor.y - info.beacon.y).abs();
            let dist = dx + dy;
            match axis {
                Axis::Row(row) => {
                    let dist_to_row = (info.sensor.y - row).abs();
                    let a = dist - dist_to_row;
                    if a > 0 {
                        let min = match limit {
                            None => info.sensor.x - a,
                            Some(_) => (info.sensor.x - a).max(0),
                        };
                        let max = match limit {
                            None => info.sensor.x + a,
                            Some(l) => (info.sensor.x + a).min(l),
                        };
                        return Some(min..=max);
                    }
                    return None;
                }
                Axis::Col(col) => {
                    let dist_to_col = (info.sensor.x - col).abs();
                    let a = dist - dist_to_col;
                    if a > 0 {
                        let min = match limit {
                            None => info.sensor.y - a,
                            Some(_) => (info.sensor.y - a).max(0),
                        };
                        let max = match limit {
                            None => info.sensor.y + a,
                            Some(l) => (info.sensor.y + a).min(l),
                        };
                        return Some(min..=max);
                    }
                    return None;
                }
            }
        })
        .sorted_by(|a, b| a.start().cmp(b.start()))
        .coalesce(coalesce_ranges)
        .map(|range| range.end() - range.start() + 1)
        .sum::<isize>()
        - 1
}

fn find_beacon_frequency(input: &[BeaconInfo], limit: isize) -> isize {
    // TODO: optimize
    let y = (0isize..)
        .find(|y| impossible_positions_for_axis(input, Axis::Row(*y), Some(limit)) < limit)
        .unwrap();
    let x = (0isize..)
        .find(|x| impossible_positions_for_axis(input, Axis::Col(*x), Some(limit)) < limit)
        .unwrap();
    x * 4000000 + y
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn solve_day_15() {
        let input = parse(INPUT);
        //assert_eq!(
        //impossible_positions_for_axis(&input, Axis::Row(10), None),
        //26
        //);
        assert_eq!(find_beacon_frequency(&input, 20), 56000011);
    }
}
