use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

pub fn spacer() {
    for _ in 0..16 {
        println!();
    }
}

pub fn get_current_time_as_epoch_milli() -> u64 {
    let start = SystemTime::now();

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

pub fn round_to_single_decimal(number: f32) -> f32 {
    (number * 10.0).round() / 10.0
}

pub fn roll() -> f32 {
    rand::thread_rng().gen_range(0.0..=100.0)
}
