#[path = "../converter.rs"]
mod converter;

use color_thief::Color;
use converter::*;

#[test]
fn test_convert_rgb_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(0, 0, 0)).unwrap(), "#000000");
}
