mod colored_string;
use colored_string::{print_string, Color};

fn main() {
    let color = Color::new(0x80, 0x00, 0x80);
    print_string("Quack!", &color)
}
