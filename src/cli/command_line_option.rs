#[path = "../converter.rs"]
mod converter;

use std::fs::File;
use std::io::prelude::*;

use clap::ArgMatches;
use color_thief::Color;

use crate::decoder::image::Image;

fn color_format(format: &str, value: &Color) {
    if format == "rgb" {
        println!("{}", value);
    } else {
        println!("{}", converter::rgb2hex(value).unwrap());
    }
}

fn palette(matches: &ArgMatches, format: &str, image: &Image) {
    let image_basename = image.file_basename();
    let colors = image.palette();

    let output = matches.value_of("output").unwrap();

    if output == "terminal" {
        for color in &colors {
            color_format(format, &color)
        }
    } else if output == "file" {
        let output_filename = format!("{}.txt", image_basename);

        let mut output_file =
            File::create(&output_filename).expect("failed to create the output file");

        output_file
            .write_all(
                colors
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .as_bytes(),
            )
            .expect("the pallete could not be exported to the output file");

        println!("File {} was successfully created!", &output_filename);
    }
}

fn dominant_color(format: &str, image: &Image) {
    let dominant_color = image.dominant_color();

    color_format(format, &dominant_color);
}

#[derive(Debug)]
pub struct CommandLineOption<'a> {
    matches: &'a ArgMatches<'a>,
    image: &'a Image<'a>,
}

impl<'a> CommandLineOption<'a> {
    pub fn new(matches: &'a ArgMatches<'a>, image: &'a Image<'a>) -> Self {
        CommandLineOption {
            matches: matches,
            image: image,
        }
    }

    pub fn handle(&self) {
        let format = self.matches.value_of("format").unwrap();

        if self.matches.occurrences_of("palette") > 0 {
            palette(&self.matches, &format, &self.image);
        }

        if self.matches.occurrences_of("dominant-color") > 0 {
            dominant_color(&format, &self.image);
        }
    }
}
