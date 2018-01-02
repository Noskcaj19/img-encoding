use errors;
use failure::Error;

use clap::{self, App, AppSettings, Arg, SubCommand};

#[derive(Debug, Clone)]
pub enum ImageColor {
    Luma,
    LumaA,
    Rgb,
    Rgba,
}

#[derive(Debug, Clone)]
pub enum InputFile {
    File(String),
    Stdin,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Encode, // Data => Img
    Decode, // Img => Data
}

#[derive(Debug, Clone)]
pub struct Options {
    input_file: InputFile,
    output_file: Option<String>,
    color_type: ImageColor,
    mode: Mode,
}

pub fn get_options() -> Result<Options, errors::OptionsError> {
    let color_types = ["luma", "lumaa", "rgb", "rgba"];

    let matches = App::new("Image Encoder")
        .version("1.0")
        .author("Noskcaj19")
        .about("Encodes arbitrary data to images")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encodes a file into an image")
                .arg(
                    Arg::with_name("color-type")
                        .help("The color type of the input image")
                        .index(1)
                        .required(true)
                        .possible_values(&color_types),
                )
                .arg(
                    Arg::with_name("in-file")
                        .help("The input file: the data to be encoded, - for standard input")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("out-file")
                        .help("The output image file")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decodes an image to the data it contains")
                .arg(
                    Arg::with_name("color-type")
                        .help("The color type of the input image")
                        .index(1)
                        .required(true)
                        .possible_values(&color_types),
                )
                .arg(
                    Arg::with_name("file")
                        .help("The input image file")
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches();
    parse_matches(&matches)
}

fn parse_matches<'a>(args: &clap::ArgMatches<'a>) -> Result<Options, errors::OptionsError> {
    Ok(match args.subcommand() {
        ("encode", Some(sub_m)) => {
            let input_file = match sub_m.value_of("in-file").ok_or(errors::OptionsError)? {
                "-" => InputFile::Stdin,
                name => InputFile::File(name.to_owned()),
            };
            let color_type = match sub_m.value_of("color-type").ok_or(errors::OptionsError)? {
                "luma" => ImageColor::Luma,
                "lumaa" => ImageColor::LumaA,
                "rgb" => ImageColor::Rgb,
                "rgba" => ImageColor::Rgba,
                _ => panic!("Unknown color type"),
            };
            Options {
                input_file,
                output_file: Some(
                    sub_m
                        .value_of("out-file")
                        .ok_or(errors::OptionsError)?
                        .to_owned(),
                ),
                color_type,
                mode: Mode::Encode,
            }
        }
        ("decode", Some(sub_m)) => {
            let input_file = match sub_m.value_of("in-file").ok_or(errors::OptionsError)? {
                "-" => InputFile::Stdin,
                name => InputFile::File(name.to_owned()),
            };
            let color_type = match sub_m.value_of("color-type").ok_or(errors::OptionsError)? {
                "luma" => ImageColor::Luma,
                "lumaa" => ImageColor::LumaA,
                "rgb" => ImageColor::Rgb,
                "rgba" => ImageColor::Rgba,
                _ => panic!("Unknown color type"),
            };
            Options {
                input_file,
                output_file: None,
                color_type,
                mode: Mode::Decode,
            }
        }
        _ => panic!("Unknown subcommand"),
    })
}
