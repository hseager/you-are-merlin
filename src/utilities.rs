use std::time::{SystemTime, UNIX_EPOCH};

pub fn spacer() {
    for _ in 0..16 {
        println!();
    }
}

pub fn get_current_time_as_epoch_milli() -> i32 {
    // Get the current time
    let now = SystemTime::now();

    // Get the duration since the Unix epoch
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Convert the duration to milliseconds
    let milliseconds_since_epoch =
        duration_since_epoch.as_secs() * 1000 + duration_since_epoch.subsec_millis() as u64;

    milliseconds_since_epoch as i32
}

pub fn round_to_single_decimal(number: f32) -> f32 {
    (number * 10.0).round() / 10.0
}
