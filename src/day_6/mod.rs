use crate::day_6::input::{Race, FINAL, FULL, TEST};

mod input;

fn compute_max_ways_to_win(race: &Race) -> usize {
    // observation is that this is a quadratic inequality with the form
    // d = w(t - w)
    // 0 = -w^2 + wt - d
    // therefore any value where d > target should work. This is an inverted parabola
    let (duration, distance) = (race.time_ms as f64, race.distance_mm as f64);
    let mut zero_1 = (duration - (duration.powi(2) - (4.0 * distance)).sqrt()) / 2.0;
    let zero_2 = (duration + (duration.powi(2) - (4.0 * distance)).sqrt()) / 2.0;

    // these are inverse parabola where each zero corresponds to a way to win the
    // race where each whole integer value inside the range is greater than the given distance

    if (zero_1.ceil() == zero_1) {
        zero_1 += 1.0
    }

    let range = (zero_1.ceil() as usize)..(zero_2.ceil() as usize);

    return range.len();
}

#[test]
fn base_cases() {
    assert_eq!(
        288,
        TEST.iter()
            .map(|t| compute_max_ways_to_win(t))
            .product::<usize>()
    )
}

#[test]
fn part_a() {
    assert_eq!(
        1660968,
        FULL.iter()
            .map(|t| compute_max_ways_to_win(t))
            .product::<usize>()
    )
}

#[test]
fn part_b() {
    assert_eq!(26499773, compute_max_ways_to_win(&FINAL))
}
