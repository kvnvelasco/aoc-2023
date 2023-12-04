use eyre::Error;
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use std::str::FromStr;

pub(super) struct Grid {
    pub width: usize,
    pub height: usize,
    model: Vec<Cell>,
}

impl Grid {
    pub(super) fn get(&self, index: Coordinate) -> Option<Cell> {
        self.model.get(index.0 + (self.width * index.1)).copied()
    }

    /// Creates a 3x3 bitfield with 1's representing digits around a coordinate
    ///  1 . #
    ///  . _ *
    ///  2 . 4
    /// For example would be represented as [4, 0, 5]
    pub(super) fn create_bitfield_around(&self, coordinate: Coordinate) -> [u32; 3] {
        let upper_left_coordinate = (coordinate + Vector(-1, -1)).unwrap_or(Coordinate(0, 0));
        let relative_coordinates = coordinate
            .get_neighbors()
            .into_iter()
            .filter_map(|c| c)
            .map(|c| Coordinate(c.0 - upper_left_coordinate.0, c.1 - upper_left_coordinate.1));
        dbg!(coordinate, upper_left_coordinate);
        let mut bitfield = [0b000, 0b000, 0b000];
        for relative_coordinate in relative_coordinates {
            if let Some(Cell::Digit(_)) = self.get(upper_left_coordinate + relative_coordinate) {
                bitfield[relative_coordinate.1] |= 1 << (2 - relative_coordinate.0)
            }
        }
        dbg!(bitfield);
        bitfield
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
pub(super) enum Cell {
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

#[derive(Debug, Copy, Clone)]
pub(super) struct Vector(pub isize, pub isize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) struct Coordinate(pub usize, pub usize);

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

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
    pub(super) fn get_neighbors(self) -> [Option<Self>; 8] {
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

    fn distance(&self, _rhs: &Self) -> f64 {
        todo!()
    }
}
