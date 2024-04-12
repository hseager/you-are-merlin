use colored::Color;

pub fn map_text_color(index: usize) -> Color {
    let colors = [
        Color::Blue,
        Color::Green,
        Color::Yellow,
        Color::Red,
        Color::Magenta,
        Color::Cyan,
        Color::BrightBlue,
        Color::BrightCyan,
        Color::BrightRed,
        Color::BrightYellow,
        Color::BrightMagenta,
        Color::BrightGreen,
    ];
    *colors
        .get(index)
        .expect("Failed to get color for text. Proabably ran out of colors (12)")
}

pub fn spacer() {
    for _ in 0..16 {
        println!();
    }
}
