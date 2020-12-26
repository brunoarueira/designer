#[path = "../converter.rs"]
mod converter;

use color_thief::Color;
use converter::*;

#[test]
fn test_convert_rgb_black_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(0, 0, 0)).unwrap(), "#000000");
}

#[test]
fn test_convert_rgb_white_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(255, 255, 255)).unwrap(), "#FFFFFF");
}

#[test]
fn test_convert_rgb_red_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(255, 0, 0)).unwrap(), "#FF0000");
}

#[test]
fn test_convert_rgb_yellowish_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(210, 155, 59)).unwrap(), "#D29B3B");
}

#[test]
fn test_convert_rgb_random_color_to_hexadecimal_version() {
    assert_eq!(rgb2hex(&Color::new(241, 245, 246)).unwrap(), "#F1F5F6");
}
