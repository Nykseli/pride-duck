mod colored_string;
use std::fs::read_to_string;

use colored_string::{print_string, Color};

fn main() {
    let colors = vec![
        Color::new(0x00, 0x00, 0x00),
        Color::new(0xA3, 0xA3, 0xA3),
        Color::new(0xFF, 0xFF, 0xFF),
        Color::new(0x80, 0x00, 0x80),
    ];

    let duck = read_to_string("duck.txt").unwrap();
    for (idx, line) in duck.lines().enumerate() {
        let color = &colors[idx % colors.len()];
        print_string(line, &color);
    }

    let color = Color::new(0x80, 0x00, 0x80);
    print_string("Quack!", &color)
}
