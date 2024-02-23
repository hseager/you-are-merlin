use colored::Color;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::BrightBlue,
        Color::BrightCyan,
        Color::BrightRed,
        Color::BrightYellow,
        Color::BrightMagenta,
        Color::BrightGreen,
    ];
    *colors.choose(&mut rng).unwrap()
}

pub fn get_random_array_index<T>(array: &[T]) -> usize {
    rand::thread_rng().gen_range(0..array.len())
}
