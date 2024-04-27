pub trait TextFormatter {
    fn text_bold(self) -> String;
    fn text_red_bold(self) -> String;
    fn text_red(self) -> String;
    fn text_cyan(self) -> String;
    fn text_green(self) -> String;
    fn text_yellow(self) -> String;
    fn text_blue(self) -> String;
    fn text_blue_bold(self) -> String;
    fn text_purple_bold(self) -> String;
    fn text_orange_bold(self) -> String;
    fn text_color(self, i: usize) -> String;
}

// Using CSS variables/custom properties in order to be able to control the colors in the web project
#[allow(dead_code)]
const RED: &str = "var(--theme-red)";
#[allow(dead_code)]
const CYAN: &str = "var(--theme-cyan)";
#[allow(dead_code)]
const GREEN: &str = "var(--theme-green)";
#[allow(dead_code)]
const YELLOW: &str = "var(--theme-yellow)";
#[allow(dead_code)]
const BLUE: &str = "var(--theme-blue)";
#[allow(dead_code)]
const MAGENTA: &str = "var(--theme-magenta)";
#[allow(dead_code)]
const SKY: &str = "var(--theme-sky)";
#[allow(dead_code)]
const PINK: &str = "var(--theme-pink)";
#[allow(dead_code)]
const AMBER: &str = "var(--theme-amber)";
#[allow(dead_code)]
const INDIGO: &str = "var(--theme-indigo)";
#[allow(dead_code)]
const PURPLE: &str = "var(--theme-purple)";
#[allow(dead_code)]
const ORANGE: &str = "var(--theme-orange)";

impl TextFormatter for &str {
    #[cfg(target_arch = "wasm32")]
    fn text_bold(self) -> String {
        format!("<strong>{}</strong>", self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_bold(self) -> String {
        use colored::Colorize;

        self.bold().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_red_bold(self) -> String {
        format!("<strong style='color: {};'>{}</strong>", RED, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_red_bold(self) -> String {
        use colored::Colorize;

        self.red().bold().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_red(self) -> String {
        format!("<span style='color: {};'>{}</span>", RED, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_red(self) -> String {
        use colored::Colorize;

        self.red().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_cyan(self) -> String {
        format!("<span style='color: {};'>{}</span>", CYAN, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_cyan(self) -> String {
        use colored::Colorize;

        self.cyan().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_green(self) -> String {
        format!("<span style='color: {};'>{}</span>", GREEN, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_green(self) -> String {
        use colored::Colorize;

        self.green().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_yellow(self) -> String {
        format!("<span style='color: {};'>{}</span>", YELLOW, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_yellow(self) -> String {
        use colored::Colorize;

        self.yellow().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_blue(self) -> String {
        format!("<span style='color: {};'>{}</span>", BLUE, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_blue(self) -> String {
        use colored::Colorize;

        self.blue().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_blue_bold(self) -> String {
        format!(
            "<span style='color: {}; font-weight: bold;'>{}</span>",
            BLUE, self
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_blue_bold(self) -> String {
        use colored::Colorize;

        self.blue().bold().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_color(self, index: usize) -> String {
        let colors = [
            RED, BLUE, GREEN, YELLOW, INDIGO, CYAN, MAGENTA, EMERALD, AMBER, SKY, PINK,
        ];

        let color = colors
            .get(index)
            .expect("Failed to get color for text. Proabably ran out of colors (12)");

        format!("<span style='color: {};'>{}</span>", color, self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_color(self, index: usize) -> String {
        use colored::{Color, Colorize};

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
        let color = colors
            .get(index)
            .expect("Failed to get color for text. Proabably ran out of colors (12)");

        self.color(*color).to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_purple_bold(self) -> String {
        format!(
            "<span style='color: {}; font-weight: bold;'>{}</span>",
            PURPLE, self
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_purple_bold(self) -> String {
        use colored::Colorize;

        self.truecolor(91, 33, 182).bold().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    fn text_orange_bold(self) -> String {
        format!(
            "<span style='color: {}; font-weight: bold;'>{}</span>",
            ORANGE, self
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn text_orange_bold(self) -> String {
        use colored::Colorize;

        self.truecolor(154, 52, 18).bold().to_string()
    }
}
