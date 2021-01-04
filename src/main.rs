#[macro_use]
extern crate clap;

use clap::{App, Arg};
use anyhow::Error;

mod cli;
mod decoder;

use cli::command_line_option::CommandLineOption;
use decoder::image::Image;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Error> {
    let matches =
        App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("brand-logo")
            .short("b")
            .long("brand-logo")
            .help("Brand logo needed to extract usefull data (e.g. most significant colors, color palette and others")
            .value_name("FILE")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("palette")
            .short("p")
            .long("palette")
            .help("Generates a palette color based on the brand logo informed")
            .takes_value(false))
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .default_value("rgb")
            .possible_values(&["rgb", "hex"])
            .help("Generates the output on the format supplied")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Print the result to your the selected output\n(when file output is selected, the entries will be concat with ',')\n")
            .default_value("terminal")
            .possible_values(&["terminal", "file"])
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("dominant-color")
            .short("d")
            .long("dominant-color")
            .help("Returns the dominant color from the image supplied")
            .conflicts_with("output")
            .takes_value(false))
        .get_matches();

    let brand_logo_filename = matches.value_of("brand-logo").unwrap();

    let image = Image::new(brand_logo_filename);

    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(std::io::stdout());

    Ok(())
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests;
