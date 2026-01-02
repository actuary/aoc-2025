use std::collections::{HashMap, HashSet};

use aoc2025::{GRID_DIRECTIONS, Grid, Position};

fn parse(data: &str) -> Grid<bool> {
    Grid::new(
        data.trim()
            .split("\n")
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect(),
    )
}

fn collect_adjacencies(grid: &Grid<bool>) -> HashMap<Position, HashSet<Position>> {
    let mut res: HashMap<Position, HashSet<Position>> = HashMap::new();

    grid.into_iter().for_each(|cell| {
        if cell.value {
            res.insert(cell.position, HashSet::new());
            grid.neighbours(&cell.position).iter().for_each(|nbr| {
                if nbr.value {
                    res.entry(cell.position).or_default().insert(nbr.position);
                }
            })
        }
    });

    res
}

pub fn neighbours(position: &Position) -> Vec<Position> {
    let mut result = vec![];
    for movement in GRID_DIRECTIONS {
        result.push(position + movement)
    }

    result
}

pub fn clean_up_rolls(adjacencies: &mut HashMap<Position, HashSet<Position>>) -> i64 {
    let removed_rolls: Vec<Position> = adjacencies
        .iter()
        .filter(|(_, nbrs)| nbrs.len() < 4)
        .map(|(roll, _)| *roll)
        .collect();

    for roll in &removed_rolls {
        for (_, v) in adjacencies.iter_mut() {
            v.remove(roll);
        }
    }

    for roll in &removed_rolls {
        adjacencies.remove(roll);
    }

    removed_rolls.len() as i64
}

pub fn print_grid(adjacencies: &HashMap<Position, HashSet<Position>>) {
    for x in 0..137 {
        for y in 0..137 {
            if adjacencies.contains_key(&Position { x, y }) {
                print!("@");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
}

pub fn part1(data: &str) -> i64 {
    let grid = parse(data);
    let mut adjacencies = collect_adjacencies(&grid);

    clean_up_rolls(&mut adjacencies)
}

pub fn part2(data: &str) -> i64 {
    let grid = parse(data);
    let mut adjacencies = collect_adjacencies(&grid);
    let start_count = adjacencies.len();

    loop {
        let removed_rolls = clean_up_rolls(&mut adjacencies);

        if removed_rolls == 0 {
            break;
        }
    }
    (start_count - adjacencies.len()) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part1(data), 13);
    }

    #[test]
    fn part2_works() {
        let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part2(data), 43);
    }
}
