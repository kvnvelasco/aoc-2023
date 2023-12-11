// Time:        47     98     66     98
// Distance:   400   1213   1011   1540

use std::num::NonZeroUsize;

pub struct Race {
    pub time_ms: usize,
    pub distance_mm: usize,
}

pub static FINAL: Race = Race {
    time_ms: 47_98_66_98,
    distance_mm: 400_1213_1011_1540,
};

pub static FULL: &'static [Race] = &[
    Race {
        time_ms: 47,
        distance_mm: 400,
    },
    Race {
        time_ms: 98,
        distance_mm: 1213,
    },
    Race {
        time_ms: 66,
        distance_mm: 1011,
    },
    Race {
        time_ms: 98,
        distance_mm: 1540,
    },
];

pub static TEST: &'static [Race] = &[
    Race {
        time_ms: 7,
        distance_mm: 9,
    },
    Race {
        time_ms: 15,
        distance_mm: 40,
    },
    Race {
        time_ms: 30,
        distance_mm: 200,
    },
];
