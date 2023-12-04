use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Card {
    id: usize,
    numbers: &'static [usize],
    winners: &'static [usize],
}

mod test_source;

fn compute_sum_of_cards(cards: &[Card]) -> usize {
    let mut sum = 0;
    for game in cards {
        let choices: HashSet<&usize> = HashSet::from_iter(game.numbers);
        let winners: HashSet<&usize> = HashSet::from_iter(game.winners);

        let count_common = winners.intersection(&choices).count();

        if count_common > 0 {
            sum += 2usize.pow(count_common as u32 - 1)
        }
    }

    sum
}

fn do_outrageous_things(cards: &[Card]) -> usize {
    let mut local_copy = cards
        .iter()
        .copied()
        .map(|c| (c, 1usize))
        .collect::<Vec<_>>();
    let mut index = 0;
    loop {
        if index == local_copy.len() {
            break;
        }
        let (game, copies) = local_copy[index];
        let choices: HashSet<&usize> = HashSet::from_iter(game.numbers);
        let winners: HashSet<&usize> = HashSet::from_iter(game.winners);

        let count_common = winners.intersection(&choices).count();
        let next_index = index + 1;
        for next_index in next_index..(next_index + count_common) {
            let handle = local_copy.get_mut(next_index).unwrap();
            handle.1 += copies
        }
        index += 1;
    }

    local_copy.iter().fold(0, |acc, (_v, copies)| acc + copies)
}

#[test]
fn test_data() {
    assert_eq!(compute_sum_of_cards(test_source::TEST_DATA), 13)
}

#[test]
fn part_1() {
    assert_eq!(compute_sum_of_cards(test_source::PROBLEM_DATA), 25004)
}

#[test]
fn part_2_test() {
    assert_eq!(do_outrageous_things(test_source::TEST_DATA), 30)
}

#[test]
fn part_2() {
    assert_eq!(do_outrageous_things(test_source::PROBLEM_DATA), 14427616)
}
