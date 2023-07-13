#[macro_use]
extern crate clap;

use clap::{Arg, Command, ArgAction};
use anyhow::Error;

mod cli;
mod decoder;

use cli::command_line_option::CommandLineOption;
use decoder::Image;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Error> {
    let matches =
        Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::new("brand-logo")
            .short('b')
            .long("brand-logo")
            .help("Brand logo needed to extract usefull data (e.g. most significant colors, color palette and others")
            .value_name("FILE")
            .required(true)
            .num_args(1))
        .arg(Arg::new("palette")
            .short('p')
            .long("palette")
            .help("Generates a palette color based on the brand logo informed")
            .num_args(0)
            .default_value("false")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .default_value("rgb")
            .value_parser(["rgb", "hex"])
            .help("Generates the output on the format supplied")
            .num_args(1))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Print the result to your the selected output\n(when file output is selected, the entries will be concat with ',')\n")
            .default_value("terminal")
            .value_parser(["terminal", "file"])
            .required(false)
            .num_args(1))
        .arg(Arg::new("dominant-color")
            .short('d')
            .long("dominant-color")
            .help("Returns the dominant color from the image supplied")
            .conflicts_with("output")
            .num_args(0)
            .action(ArgAction::SetTrue))
        .get_matches();

    let brand_logo_filename = matches.get_one::<String>("brand-logo").unwrap();

    let image = Image::new(brand_logo_filename);

    let command_line_option = CommandLineOption::new(&matches, &image);

    command_line_option.handle(std::io::stdout());

    Ok(())
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests;
