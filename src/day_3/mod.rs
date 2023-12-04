use eyre::{Context, Error};
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::{Add, Index};
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Vector(isize, isize);

#[derive(Copy, Clone)]
struct Coordinate(usize, usize);

impl Add<Vector> for Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, rhs: Vector) -> Self::Output {
        Some(Self(
            self.0.checked_add_signed(rhs.0)?,
            self.1.checked_add_signed(rhs.1)?,
        ))
    }
}

impl Coordinate {
    fn get_neighbors(self) -> [Option<Self>; 8] {
        [
            self + Vector(-1, -1),
            self + Vector(0, -1),
            self + Vector(1, -1),
            self + Vector(-1, 0),
            self + Vector(1, 0),
            self + Vector(-1, 1),
            self + Vector(0, 1),
            self + Vector(1, 1),
        ]
    }

    fn distance(&self, rhs: &Self) -> f64 {
        todo!()
    }
}

struct Grid {
    width: usize,
    height: usize,
    model: Vec<Cell>,
}

impl Grid {
    fn get(&self, index: Coordinate) -> Option<Cell> {
        self.model.get(index.0 + (self.width * index.1)).copied()
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();

        let model: Vec<Cell> = lines
            .flat_map(|line| line.trim().chars())
            .map(|char| match char {
                '0'..='9' => Cell::Digit(char.to_digit(10).unwrap() as usize),
                '.' => Cell::Empty,
                char => Cell::Symbol(char),
            })
            .collect();

        let width = s.trim().lines().next().unwrap().len();
        let height = model.chunks(width).count();
        return Ok(Self {
            width,
            model,
            height,
        });
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, line) in self.model.chunks(self.width).enumerate() {
            for cell in line {
                cell.fmt(f)?;
            }

            if idx < self.height - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Gear,
    Digit(usize),
    Symbol(char),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Gear => f.write_char('*'),
            Cell::Empty => f.write_char('.'),
            Cell::Digit(d) => Display::fmt(d, f),
            Cell::Symbol(x) => Display::fmt(x, f),
        }
    }
}

#[test]
fn grid_parsing() {
    let grid: Grid = r"
    467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    "
    .parse()
    .unwrap();

    assert_eq!(
        grid.to_string(),
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
    )
}

fn get_model_numbers(grid: Grid) -> eyre::Result<Vec<usize>> {
    let mut esults = vec![];
    let iterator = (0..grid.height)
        .into_iter()
        .map(|y| ((0..grid.width).into_iter(), y));

    for (row, y) in iterator {
        let mut current_digits = vec![];
        let mut should_flush = false;

        // accumulate a number in current number, and see if it should flush
        macro_rules! flush {
            () => {{
                if should_flush && current_digits.len() > 0 {
                    esults.push(
                        current_digits
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (idx, v)| acc + v * 10usize.pow(idx as u32)),
                    );
                }
                current_digits.clear();
                should_flush = false;
            };};
        }

        for x in row {
            let cell = grid.get(Coordinate(x, y));
            match cell {
                None => flush!(),
                Some(Cell::Empty | Cell::Symbol(_) | Cell::Gear) => {
                    flush!();
                }
                Some(Cell::Digit(v)) => {
                    current_digits.push(v);
                    should_flush = should_flush
                        || Coordinate(x, y)
                            .get_neighbors()
                            .into_iter()
                            .filter_map(|x| x)
                            .any(|c| matches!(grid.get(c), Some(Cell::Symbol(_))));
                }
            }
        }

        flush!();
    }

    Ok(esults)
}

fn get_gears(grid: Grid) -> eyre::Result<Vec<usize>> {
    let mut esults = vec![];
    let iterator = (0..grid.height)
        .into_iter()
        .map(|y| ((0..grid.width).into_iter(), y));

    for (row, y) in iterator {
        let mut current_digits = vec![];
        let mut should_flush = false;

        // accumulate a number in current number, and see if it should flush
        macro_rules! flush {
            () => {{
                if should_flush && current_digits.len() > 0 {
                    esults.push(
                        current_digits
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (idx, v)| acc + v * 10usize.pow(idx as u32)),
                    );
                }
                current_digits.clear();
                should_flush = false;
            };};
        }

        for x in row {
            let cell = grid.get(Coordinate(x, y));
            match cell {
                None => flush!(),
                Some(Cell::Empty | Cell::Symbol(_) | Cell::Gear) => {
                    flush!();
                }
                Some(Cell::Digit(v)) => {
                    current_digits.push(v);
                    let neighbors = Coordinate(x, y).get_neighbors();
                    // we only search for the next link gear. We only want the coordiantes that we care about next
                    let search_space_for_next_gear = neighbors
                        .iter()
                        .copied()
                        .filter_map(|x| x)
                        .filter(|c| c.1 >= y)
                        .map(|c| grid.get(c))
                        .filter_map(|c| c)
                        .filter(|c| matches!(c, Cell::Gear));

                    for gear in search_space_for_next_gear {
                        // we want to search all the neighbors to make sure that
                        // 1. there are only two numbers adjacent to the gear
                        // 2. if there are more than two numbers, they must be on the same row
                        // 3. If the number is on our row, there can only be one other.
                    }

                    should_flush = should_flush
                        || Coordinate(x, y)
                            .get_neighbors()
                            .into_iter()
                            .filter_map(|x| x)
                            .any(|c| matches!(grid.get(c), Some(Cell::Symbol(_))));
                }
            }
        }

        flush!();
    }

    Ok(esults)
}

#[test]
fn part_count() {
    let grid: Grid = r"
    467..114..
    ...*......
    ..35...633
    .......#..
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "
    .parse()
    .unwrap();

    let model_numbers = get_model_numbers(grid).unwrap();
    assert_eq!(model_numbers.into_iter().sum::<usize>(), 4361)
}

#[test]
fn part_1() {
    let grid: Grid = include_str!("./input.txt").parse().unwrap();

    let model_numbers = get_model_numbers(grid).unwrap();

    assert_eq!(model_numbers.into_iter().sum::<usize>(), 533775)
}
