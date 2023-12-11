use crate::day_5::input_data::{Key, Maps};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::ops::Range;
use std::thread::{available_parallelism, scope, spawn};

mod input_data;

/// take the set of all maps and find the chain of keys that goes from `location` to `seed`
fn build_key_chain(maps: &Maps) -> Vec<Key> {
    let keys = maps.keys();

    let mut chain = vec![];
    // location link
    let mut current_target = "location";
    let mut runs = maps.len() + 1;
    loop {
        runs -= 1;
        if runs == 0 {
            panic!("Exhausted chain before finding link from seed - location")
        }
        let target = keys
            .clone()
            .find(|k| k.1 == current_target)
            .expect("Target could not be found in map");
        chain.insert(0, *target);

        if target.0 == "seed" {
            break;
        } else {
            current_target = target.0
        }
    }
    return chain;
}

#[test]
fn test_data_links() {
    let maps = input_data::test::get_maps();
    let chain = build_key_chain(&maps);

    assert_eq!(
        chain,
        vec![
            ("seed", "soil"),
            ("soil", "fertilizer"),
            ("fertilizer", "water"),
            ("water", "light"),
            ("light", "temperature"),
            ("temperature", "humidity"),
            ("humidity", "location")
        ]
    )
}

fn find_location_for_seed(seed: usize, lookup_chain: &[Key], maps: &Maps) -> usize {
    let mut current = seed;
    for link in lookup_chain {
        let mapping = maps.get(link).expect("No key in maps");
        let index = mapping
            .binary_search_by(|(id, c)| {
                if c.contains(&current) {
                    Ordering::Equal
                } else if c.end <= current {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .ok()
            .map(|index| mapping.get(index))
            .flatten();

        if let Some((key, range)) = index {
            let floor = current - range.start;
            current = floor + key;
        }
    }
    return current;
}

#[test]
fn find_location_for_seeds() {
    let maps = input_data::test::get_maps();
    let chain = build_key_chain(&maps);

    let location = find_location_for_seed(79, &chain, &maps);
    assert_eq!(location, 82);

    let location = find_location_for_seed(13, &chain, &maps);
    assert_eq!(location, 35)
}

#[test]
fn part_a() {
    let seeds = input_data::full::SEEDS;
    let maps = input_data::full::get_maps();
    let chain = build_key_chain(&maps);

    let min = seeds
        .iter()
        .map(|x| find_location_for_seed(*x, &chain, &maps))
        .min();

    assert_eq!(min, Some(457535844));
}

#[ignore]
#[test]
fn part_b() {
    use rayon::prelude::*;
    let ranges = input_data::full::SEED_RANGE;

    let ranges = ranges.iter().cloned().flat_map(|f| f.into_iter());

    let maps = input_data::full::get_maps();
    let chain = build_key_chain(&maps);

    let value = ranges
        .par_bridge()
        .map(|g| find_location_for_seed(g, &chain, &maps))
        .min();

    assert_eq!(value, Some(41222968));
}
