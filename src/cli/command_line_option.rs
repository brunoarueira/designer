#[path = "../converter.rs"]
mod converter;

use std::fs::File;
use std::io::Write;

use clap::ArgMatches;
use color_thief::Color;

use crate::decoder::Image;

fn color_format(format: &str, value: &Color) -> String {
    if format == "rgb" {
        format!("{}", value)
    } else {
        converter::rgb2hex(value).unwrap()
    }
}

fn palette(matches: &ArgMatches, format: &str, image: &Image, mut writer: impl Write) {
    match image.palette() {
        Some(colors) => {
            let image_basename = image.file_basename();

            let output = matches.get_one::<String>("output").unwrap();

            if output == "terminal" {
                for color in &colors {
                    writeln!(writer, "{}", color_format(format, color)).unwrap();
                }
            } else if output == "file" {
                let output_filename = format!("{}.txt", image_basename);

                let mut output_file =
                    File::create(&output_filename).expect("failed to create the output file");

                output_file
                    .write_all(
                        colors
                            .iter()
                            .map(|c| color_format(format, c))
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

#[derive(Debug)]
pub struct CommandLineOption<'a> {
    matches: &'a ArgMatches,
    image: &'a Image<'a>,
}

impl<'a> CommandLineOption<'a> {
    pub fn new(matches: &'a ArgMatches, image: &'a Image<'a>) -> Self {
        CommandLineOption {
            matches,
            image,
        }
    }

    fn dominant_color(&self, format: &str, mut writer: impl Write) {
        let dominant_color = self.image.dominant_color();

        writeln!(writer, "{}", color_format(format, &dominant_color)).unwrap();
    }

    pub fn handle(&self, mut writer: impl Write) {
        let format = self.matches.get_one::<String>("format").unwrap();

        let palette_value: bool = self.matches.try_contains_id("palette").is_ok();

        if palette_value {
            palette(self.matches, format, self.image, &mut writer);
        }

        let dominant_color_value: bool = self.matches.try_contains_id("dominant-color").is_ok();

        if dominant_color_value {
            self.dominant_color(format, &mut writer);
        }
    }
}
