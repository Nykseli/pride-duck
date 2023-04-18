pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub fn print_string(text: &str, color: &Color) {
    let Color { r, b, g } = color;
    // No octal escape so we need hex \x1b
    println!("\x1b[38;2;{r};{g};{b}m{text}\x1b[m");
}
