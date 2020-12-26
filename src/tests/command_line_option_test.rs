#[path = "test_helpers.rs"]
mod test_helpers;

use std::fs::read_to_string;

use clap::{App, Arg};

use test_helpers::fixture_path;

use crate::cli::command_line_option::CommandLineOption;
use crate::decoder::image::Image;

#[test]
fn test_color_palette_output_to_stdout() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture.png");
    let image = Image::new(&path);
    let matches = App::new("myprog")
        .arg(Arg::with_name("b").short("b").takes_value(true))
        .arg(Arg::with_name("format").short("f").takes_value(true))
        .arg(Arg::with_name("palette").short("p"))
        .arg(
            Arg::with_name("output")
                .short("o")
                .default_value("terminal")
                .takes_value(true),
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
fn test_color_palette_output_to_file() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture.png");
    let image = Image::new(&path);
    let matches = App::new("myprog")
        .arg(Arg::with_name("b").short("b").takes_value(true))
        .arg(Arg::with_name("format").short("f").takes_value(true))
        .arg(Arg::with_name("palette").short("p"))
        .arg(
            Arg::with_name("output")
                .short("o")
                .default_value("file")
                .takes_value(true),
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(
        read_to_string(fixture_path("image-fixture.txt")).unwrap(),
        "rgb(116,116,116),rgb(4,4,4),rgb(252,228,132),rgb(152,172,68),rgb(60,172,128),rgb(120,172,60),rgb(120,172,60),rgb(120,172,60),rgb(120,172,60)\n"
    );
}

#[test]
fn test_color_palette_output_from_not_found_image_to_file() {
    let mut result = Vec::new();
    let path = fixture_path("image-fixture2.png");
    let image = Image::new(&path);
    let matches = App::new("myprog")
        .arg(Arg::with_name("b").short("b").takes_value(true))
        .arg(Arg::with_name("format").short("f").takes_value(true))
        .arg(Arg::with_name("palette").short("p"))
        .arg(
            Arg::with_name("output")
                .short("o")
                .default_value("file")
                .takes_value(true),
        )
        .get_matches_from(vec!["myprog", "-b", &path, "-f", "rgb", "-p"]);
    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(&mut result);

    assert_eq!(result, b"The palette could not be extracted from image\n");
}
