use eyre::{bail, eyre, Context};
use std::collections::HashMap;

use std::str::FromStr;

/// You're launched high into the atmosphere! The apex of your trajectory just
/// barely reaches the surface of a large island floating in the sky. You gently
/// land in a fluffy pile of leaves. It's quite cold, but you don't see much snow.
/// An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for the lack
/// of snow. He'll be happy to explain the situation, but it's a bit of a walk, so
/// you have some time. They don't get many visitors up here; would you like to play
/// a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either red,
/// green, or blue. Each time you play this game, he will hide a secret number of
/// cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will
/// reach into the bag, grab a handful of random cubes, show them to you, and
/// then put them back in the bag. He'll do this a few times per game.
///
/// You play several games and record the information from each game (your puzzle input).
/// Each game is listed with its ID number (like the 11 in Game 11: ...)
/// followed by a semicolon-separated list of subsets of cubes that were revealed
/// from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again).
/// The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes,
/// and 6 blue cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained
/// only 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been
/// loaded with that configuration. However, game 3 would have been impossible because at one
/// point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible
/// because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that
/// would have been possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded with only
/// 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

macro_rules! unwrap_continue {
    ($ex:expr) => {{
        if let Some(v) = $ex {
            v
        } else {
            continue;
        }
    }};
}

#[derive(Debug)]
struct Game {
    id: usize,
    attempts: Vec<Attempt>,
}

#[derive(Debug)]
struct Attempt(HashMap<&'static str, usize>);

impl FromStr for Attempt {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self(Default::default());
        let items = s.split(",");
        'color_loop: for color in ["red", "green", "blue"] {
            for item in items.clone() {
                let item = item.trim();

                let candidate = unwrap_continue!(item.strip_suffix(color));
                let digit_character = unwrap_continue!(candidate.trim().parse::<usize>().ok());

                this.0.insert(color, digit_character);
                continue 'color_loop;
            }
        }

        if this.0.len() == 0 {
            bail!("Could not parse out attempt from {s} got {:?}", &this.0);
        }

        return Ok(this);
    }
}

#[test]
fn parse_attempt() {
    let attempt = Attempt::from_str("3 red, 5 blue, 6 green");
    let map = attempt.unwrap().0;

    assert_eq!(map["red"], 3);
    assert_eq!(map["blue"], 5);
    assert_eq!(map["green"], 6);
}

impl FromStr for Game {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, game) = s
            .split_once(':')
            .ok_or(eyre!("Incorrect format of {s}. Expects single :"))?;

        let attempts = game
            .split(';')
            .map(Attempt::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let game_id = id
            .strip_prefix("Game")
            .ok_or(eyre!("Incorrect game prefix {id}"))?
            .trim()
            .parse()
            .context("Not a valid game id")?;

        Ok(Self {
            id: game_id,
            attempts,
        })
    }
}

fn compute_valid_games(games: Vec<Game>) -> usize {
    games
        .iter()
        .filter_map(|g| {
            for attempt in &g.attempts {
                for (color, roll) in attempt.0.iter() {
                    match *color {
                        "red" if *roll > 12 => return None,
                        "green" if *roll > 13 => return None,
                        "blue" if *roll > 14 => return None,
                        _ => continue,
                    }
                }
            }

            Some(g.id)
        })
        .sum()
}

fn compute_powers(games: Vec<Game>) -> usize {
    games
        .iter()
        .map(|g| {
            let max_values = g.attempts.iter().fold(
                (0usize, 0usize, 0usize),
                |(mut red, mut green, mut blue), g| {
                    red = red.max(g.0.get("red").copied().unwrap_or_default());
                    green = green.max(g.0.get("green").copied().unwrap_or_default());
                    blue = blue.max(g.0.get("blue").copied().unwrap_or_default());

                    (red, green, blue)
                },
            );

            return max_values.0 * max_values.1 * max_values.2;
        })
        .sum()
}

#[test]
fn naive_case() {
    let game = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "
    .trim()
    .lines()
    .map(Game::from_str)
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    assert_eq!(compute_valid_games(game), 8)
}

#[test]
fn first_full_case() {
    let game = include_str!("./input_1.txt")
        .trim()
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(compute_valid_games(game), 2632)
}

#[test]
fn second_full_test_case() {
    let game = include_str!("./input_1.txt")
        .trim()
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(compute_powers(game), 69629)
}
