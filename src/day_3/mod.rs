use eyre::{Context, Error};
use std::collections::{BTreeSet, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::{Add, Index};
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Vector(isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
                '*' => Cell::Gear,
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
                Display::fmt(cell, f)?;
            }

            if idx < self.height - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
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
    let iterator = (0..grid.height)
        .into_iter()
        .flat_map(move |y| ((0..grid.width).into_iter().map(move |x| Coordinate(x, y))));
    let mut output = vec![];
    {
        let mut gears = HashSet::<Coordinate>::new();
        for coordinate in iterator.clone() {
            if !matches!(grid.get(coordinate), Some(Cell::Gear)) {
                continue;
            }

            let neighbors = coordinate.get_neighbors().into_iter().filter_map(|c| c);

            // if there are any other symbols around us, we are not a gear.
            if neighbors
                .clone()
                .any(|c| matches!(grid.get(c), Some(Cell::Symbol(_))))
            {
                continue;
            }

            // find all of the neighboring digits
            let neighbor_number_cells = neighbors
                .clone()
                .filter(|c| matches!(grid.get(*c), Some(Cell::Digit(_))))
                .collect::<Vec<_>>();

            // if there is only 1 digit, this is not a gear.
            if neighbor_number_cells.len() < 2 {
                continue;
            }

            let bitfield = {
                let upper_left_coordinate =
                    (coordinate + Vector(-1, -1)).unwrap_or(Coordinate(0, 0));
                let coordinates = neighbor_number_cells
                    .iter()
                    .map(|c| {
                        Coordinate(c.0 - upper_left_coordinate.0, c.1 - upper_left_coordinate.1)
                    })
                    .collect::<Vec<_>>();

                let mut bitfield = [0b000, 0b000, 0b000];
                for coordinate in coordinates {
                    bitfield[coordinate.1] |= 1 << (2 - coordinate.0)
                }

                bitfield
            };

            macro_rules! bit {
                ($t:literal, $m:literal, $b:literal) => {
                    &[$t, $m, $b]
                };
            }

            macro_rules! splat {
                (b|$v:literal) => {
                    bit!(7, 0, $v)
                        | bit!(6, 0, $v)
                        | bit!(4, 0, $v)
                        | bit!(3, 0, $v)
                        | bit!(2, 0, $v)
                        | bit!(1, 0, $v)
                        | bit!(0, 4, $v)
                        | bit!(0, 1, $v)
                };
                (t|$v:literal) => {
                    bit!($v, 0, 7)
                        | bit!($v, 0, 6)
                        | bit!($v, 0, 4)
                        | bit!($v, 0, 3)
                        | bit!($v, 0, 2)
                        | bit!($v, 0, 1)
                        | bit!($v, 4, 0)
                        | bit!($v, 1, 0)
                };

                ($mode:tt) => {
                    splat!($mode | 7)
                        | splat!($mode | 6)
                        | splat!($mode | 4)
                        | splat!($mode | 3)
                        | splat!($mode | 1)
                };
                () => {
                    splat!(t) | splat!(b)
                };
            }

            // there are a limited number of valid configurations
            match &bitfield {
                splat!() | bit!(0, 5, 0) | bit!(5, 0, 0) | bit!(0, 0, 5) => {
                    let numbers = neighbor_number_cells
                        .iter()
                        .map(|x| find_number_in_grid_from(&grid, *x))
                        .collect::<HashSet<_>>();
                    dbg!(coordinate, &numbers);
                    if (numbers.len() == 1) {
                        output.push(numbers.iter().next().unwrap().pow(2));
                    } else {
                        assert_eq!(numbers.len(), 2);
                        output.push(numbers.iter().product())
                    }
                }
                _ => {}
            }
        }
    }

    Ok(output)
}

fn find_number_in_grid_from(grid: &Grid, pos: Coordinate) -> usize {
    let mut pointer = pos;
    loop {
        if (pointer + Vector(-1, 0)).is_some_and(|c| matches!(grid.get(c), Some(Cell::Digit(_)))) {
            pointer = (pointer + Vector(-1, 0)).unwrap()
        } else {
            break;
        }
    }

    let mut digits = vec![];
    loop {
        if let Some(Cell::Digit(digit)) = grid.get(pointer) {
            pointer = (pointer + Vector(1, 0)).unwrap();
            digits.insert(0, digit)
        } else {
            break;
        }
    }

    digits
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + val * 10usize.pow(idx as u32))
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
    assert_eq!(model_numbers.into_iter().sum::<usize>(), 1889)
}

#[test]
fn part_1() {
    let grid: Grid = include_str!("./input.txt").parse().unwrap();

    let model_numbers = get_model_numbers(grid).unwrap();

    assert_eq!(model_numbers.into_iter().sum::<usize>(), 196643)
}

#[test]
fn part_2() {
    let grid: Grid = include_str!("./input.txt").parse().unwrap();
    let model_numbers = get_gears(grid).unwrap();

    assert_eq!(model_numbers.into_iter().sum::<usize>(), 78236071)
}
