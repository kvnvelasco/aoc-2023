use std::collections::HashSet;
use std::fmt::{Debug, Display, Write};
use std::ops::Add;
use std::str::FromStr;

use model::*;

mod model;

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
    let mut unique_y_coordinates: HashSet<isize> = Default::default();
    for coordinate in iterator.clone() {
        if !matches!(grid.get(coordinate), Some(Cell::Gear)) {
            continue;
        }

        let bitfield = grid.create_bitfield_around(coordinate);
        // there are a computable number of valid states for the bitfield
        // Of 256 possible states, 80 are valid where only two numbers
        // surround a gear. We can construct these valid states using
        // a recursive macro
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
                let vectors = bitfield_to_vectorfield(bitfield);
                unique_y_coordinates.clear();
                unique_y_coordinates.extend(vectors.iter().map(|v| v.1));

                if unique_y_coordinates.len() == 1 {
                    output.push(
                        vectors
                            .into_iter()
                            .filter_map(|v| coordinate + v)
                            .map(|c| find_number_in_grid_from(&grid, c))
                            .product(),
                    );
                } else {
                    assert_eq!(unique_y_coordinates.len(), 2);
                    let mut iterator = unique_y_coordinates.iter();
                    let (y1, y2) = (*iterator.next().unwrap(), *iterator.next().unwrap());
                    let first = vectors.iter().find(|v| v.1 == y1).unwrap();

                    let second = vectors.iter().find(|v| v.1 == y2).unwrap();

                    output.push(
                        find_number_in_grid_from(&grid, (coordinate + *first).unwrap())
                            * find_number_in_grid_from(&grid, (coordinate + *second).unwrap()),
                    );
                }
            }
            _ => {}
        }
    }

    Ok(output)
}

// We can convert the bitfield into a set of relative vectors around
// its center detecting where the 1s are om the number and converting it into
// a vector
fn bitfield_to_vectorfield(bitfield: [u32; 3]) -> Vec<Vector> {
    let mut output = vec![];
    for y in -1isize..2 {
        let y_value = bitfield[(y + 1) as usize];
        for x in -1..2 {
            let x_value = (y_value >> 1 - x) & 0b001;
            if x_value == 1 {
                output.push(Vector(x, y))
            }
        }
    }
    output
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
