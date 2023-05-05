#[path = "test_helpers.rs"]
mod test_helpers;

use color_thief::Color;

use crate::decoder::Image;
use test_helpers::fixture_path;

#[test]
fn test_color_palette_from_supplied_image() {
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);

    assert_eq!(
        image.palette().unwrap(),
        vec![
            Color::new(116, 116, 116),
            Color::new(4, 4, 4),
            Color::new(252, 228, 132),
            Color::new(152, 172, 68),
            Color::new(60, 172, 128),
            Color::new(120, 172, 60),
            Color::new(120, 172, 60),
            Color::new(120, 172, 60),
            Color::new(120, 172, 60)
        ]
    )
}

#[test]
fn test_must_return_an_empty_color_palette_on_not_found_image() {
    let image = Image::new("image2.png");

    assert_eq!(image.palette(), None)
}

#[test]
#[should_panic]
fn test_invalid_file_opening() {
    let path = fixture_path("image-fixture-corrupted.png");
    let image = Image::new(&path);

    assert_eq!(image.palette(), None)
}

#[test]
fn test_file_basename_without_extension() {
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);

    assert_eq!(image.file_basename(), "image-fixture-rgba")
}

#[test]
fn test_dominant_color_from_rgba_images() {
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);

    assert_eq!(image.dominant_color(), Color::new(116, 116, 116));
}

#[test]
fn test_dominant_color_from_rgb_images() {
    let path = fixture_path("image-fixture-rgb.png");
    let image = Image::new(&path);

    assert_eq!(image.dominant_color(), Color::new(68, 68, 68));
}
