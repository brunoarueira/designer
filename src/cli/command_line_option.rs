#[path = "../converter.rs"]
mod converter;

use std::fs::File;
use std::io::Write;

use clap::ArgMatches;
use color_thief::Color;

use crate::decoder::image::Image;

fn color_format(format: &str, value: &Color, mut writer: impl Write) {
    if format == "rgb" {
        writeln!(writer, "{}", value).unwrap();
    } else {
        writeln!(writer, "{}", converter::rgb2hex(value).unwrap()).unwrap();
    }
}

fn palette(matches: &ArgMatches, format: &str, image: &Image, mut writer: impl Write) {
    match image.palette() {
        Some(colors) => {
            let image_basename = image.file_basename();

            let output = matches.value_of("output").unwrap();

            if output == "terminal" {
                for color in &colors {
                    color_format(format, &color, &mut writer);
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
                    .expect("the palette could not be exported to the output file");

                writeln!(
                    writer,
                    "File {} was successfully created!",
                    &output_filename
                )
                .unwrap();
            }
        }
        None => {
            writeln!(writer, "The palette could not be extracted from image").unwrap();
        }
    }
}

fn dominant_color(format: &str, image: &Image, writer: impl Write) {
    let dominant_color = image.dominant_color();

    color_format(format, &dominant_color, writer);
}

#[derive(Debug)]
pub struct CommandLineOption<'a> {
    matches: &'a ArgMatches<'a>,
    image: &'a Image<'a>,
}

impl<'a> CommandLineOption<'a> {
    pub fn new(matches: &'a ArgMatches<'a>, image: &'a Image<'a>) -> Self {
        CommandLineOption {
            matches,
            image,
        }
    }

    pub fn handle(&self, mut writer: impl Write) {
        let format = self.matches.value_of("format").unwrap();

        if self.matches.occurrences_of("palette") > 0 {
            palette(&self.matches, &format, &self.image, &mut writer);
        }

        if self.matches.occurrences_of("dominant-color") > 0 {
            dominant_color(&format, &self.image, &mut writer);
        }
    }
}
