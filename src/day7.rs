use std::collections::HashMap;

#[derive(Debug, Default)]
struct Dir {
    files: HashMap<String, Dir>,
    total_size: usize,
}

#[derive(Debug)]
enum Append {
    AddFile { size: usize },
    AddDir { name: String },
}

impl Append {
    fn parse(input: &str) -> Self {
        if input.starts_with("dir ") {
            return Append::AddDir {
                name: input[4..].to_string(),
            };
        }
        let mut splitted = input.split(" ");
        let size: usize = splitted.next().unwrap().parse().unwrap();
        Append::AddFile { size }
    }
}

impl Dir {
    fn append(&mut self, path: &[String], append: Append) {
        match append {
            Append::AddFile { size, .. } => self.total_size += size,
            _ => (),
        }

        if path.is_empty() {
            match append {
                Append::AddDir { name } => {
                    self.files.insert(name, Dir::default());
                }
                _ => (),
            }
        } else {
            let dir = self.files.get_mut(&path[0]).unwrap();
            dir.append(&path[1..], append);
        }
    }

    fn scan_part_one(&self, prev: usize) -> usize {
        let children: usize = self
            .files
            .iter()
            .map(|(_k, dir)| dir.scan_part_one(0))
            .sum();
        if self.total_size <= 100000 {
            prev + children + self.total_size
        } else {
            prev + children
        }
    }

    fn scan_part_two(&self, wanted_space: usize) -> Option<usize> {
        if self.total_size < wanted_space {
            return None;
        }

        self.files
            .iter()
            .filter_map(|(_k, dir)| dir.scan_part_two(wanted_space))
            .chain(Some(self.total_size))
            .min()
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Dir {
    let mut root = Dir::default();
    let mut full_path = vec!["/".to_string()];
    for line in input.lines() {
        if line == "$ ls" {
            continue;
        }
        if line.starts_with("$ cd") {
            let add_to_path = line.split(' ').skip(2).next().unwrap();
            match add_to_path {
                "/" => (),
                ".." => {
                    full_path.pop();
                }
                path => full_path.push(path.to_string()),
            }
            continue;
        }
        root.append(&full_path[1..], Append::parse(line));
    }
    root
}

#[aoc(day7, part1)]
fn part1(input: &Dir) -> usize {
    input.scan_part_one(0)
}

#[aoc(day7, part2)]
fn part2(input: &Dir) -> usize {
    let free_space = 70000000 - input.total_size;
    let wanted_space = 30000000 - free_space;
    input.scan_part_two(wanted_space).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn solve_day_7() {
        let input = parse(INPUT);
        dbg!(&input);
        assert_eq!(part1(&input), 95437);
        assert_eq!(part2(&input), 24933642);
    }
}
