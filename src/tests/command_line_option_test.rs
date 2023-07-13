#[path = "test_helpers.rs"]
mod test_helpers;

use std::fs::read_to_string;

use clap::{Arg, ArgAction, Command};

use test_helpers::fixture_path;

use crate::cli::command_line_option::CommandLineOption;
use crate::decoder::Image;

#[test]
fn test_color_palette_output_as_rgb_to_stdout() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("palette").short('p').action(ArgAction::SetTrue))
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("terminal")
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(
        result,
        b"rgb(116,116,116)\nrgb(4,4,4)\nrgb(252,228,132)\nrgb(152,172,68)\nrgb(60,172,128)\nrgb(120,172,60)\nrgb(120,172,60)\nrgb(120,172,60)\nrgb(120,172,60)\n"
    );
}

#[test]
fn test_color_palette_output_as_hex_to_stdout() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("palette").short('p').action(ArgAction::SetTrue))
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("terminal")
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "hex", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(
        result,
        b"#747474\n#040404\n#FCE484\n#98AC44\n#3CAC80\n#78AC3C\n#78AC3C\n#78AC3C\n#78AC3C\n"
    );
}

#[test]
fn test_dominant_color_as_rgb_to_stdout() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b').required(true))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("dominant-color").short('d').action(ArgAction::SetTrue))
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-d"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(result, b"rgb(116,116,116)\n");
}

#[test]
fn test_dominant_color_as_hex_to_stdout() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("dominant-color").short('d').action(ArgAction::SetTrue))
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "hex", "-d"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(result, b"#747474\n");
}

#[test]
fn test_color_palette_output_as_rgb_to_file() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("palette").short('p').action(ArgAction::SetTrue))
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("file")
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(
        read_to_string(fixture_path("image-fixture-rgb.txt")).unwrap(),
        "rgb(116,116,116),rgb(4,4,4),rgb(252,228,132),rgb(152,172,68),rgb(60,172,128),rgb(120,172,60),rgb(120,172,60),rgb(120,172,60),rgb(120,172,60)\n"
    );
}

#[test]
fn test_color_palette_output_as_hex_to_file() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture-rgba.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("palette").short('p').action(ArgAction::SetTrue))
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("file")
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "hex", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(
        read_to_string(fixture_path("image-fixture-hex.txt")).unwrap(),
        "#747474,#040404,#FCE484,#98AC44,#3CAC80,#78AC3C,#78AC3C,#78AC3C,#78AC3C\n"
    );
}

#[test]
fn test_color_palette_output_from_not_found_image_to_file() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture2.png");
    let image = Image::new(&path);
    let matches = Command::new("myprog")
        .arg(Arg::new("brand-logo").short('b'))
        .arg(Arg::new("format").short('f'))
        .arg(Arg::new("palette").short('p').action(ArgAction::SetTrue))
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("file")
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-p", "-o", "file"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(result, b"The palette could not be extracted from image\n");
}
