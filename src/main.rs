extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate image;

use failure::Error;

use std::io::{self, Read};
use std::fs::File;

mod errors;
mod options;

// Copied from image::color
pub fn num_components(c: options::ImageColor) -> u32 {
    use options::ImageColor;
    match c {
        ImageColor::Gray => 1,
        ImageColor::GrayA => 2,
        ImageColor::Rgb => 3,
        ImageColor::RgbA => 4,
    }
}

fn pad_data(mut data: Vec<u8>, length: usize) -> Vec<u8> {
    let to_repeat = (length - (data.len() % length)) as usize;
    data.extend_from_slice(&mut "\x00".repeat(to_repeat).as_bytes());
    data
}

// Improve formula to minimize size
fn img_dimensions(data_len: usize, color: options::ImageColor) -> (u32, u32) {
    let size = f64::from(data_len as f64 / num_components(color) as f64)
        .ceil()
        .sqrt()
        .ceil() as u32;
    (size, size)
}

fn get_stdin() -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_file(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_input(input: options::InputFile) -> Result<Vec<u8>, Error> {
    match input {
        options::InputFile::Stdin => Ok(get_stdin()?),
        options::InputFile::File(file_path) => Ok(get_file(&file_path)?),
    }
}

fn encode(options: options::Options) -> Result<(), Error> {
    ensure!(options.output_file.is_some(), "No output file provided");
    // `color_type` guaranteed to be some for encode
    let color_type = options.color_type.unwrap();

    let data = get_input(options.input_file)?;
    let (width, height) = img_dimensions(data.len(), color_type);

    let data = pad_data(data, (width * height * num_components(color_type)) as usize);

    // `output_file` is guaranteed to be Some for encode
    let fout = &mut std::fs::File::create(options.output_file.unwrap())?;

    // Ugly mess to appease typecheck
    use options::ImageColor;
    match color_type {
        ImageColor::Gray => {
            let imgbuf: image::ImageBuffer<image::Luma<_>, Vec<_>> =
                image::ImageBuffer::from_raw(width, height, data.to_owned())
                    .ok_or(errors::EncodingError)?;
            image::ImageLuma8(imgbuf).save(fout, image::PNG)?;
        }
        ImageColor::GrayA => {
            let imgbuf: image::ImageBuffer<image::LumaA<_>, Vec<_>> =
                image::ImageBuffer::from_raw(width, height, data.to_owned())
                    .ok_or(errors::EncodingError)?;
            image::ImageLumaA8(imgbuf).save(fout, image::PNG)?;
        }
        ImageColor::Rgb => {
            let imgbuf: image::ImageBuffer<image::Rgb<_>, Vec<_>> =
                image::ImageBuffer::from_raw(width, height, data.to_owned())
                    .ok_or(errors::EncodingError)?;
            image::ImageRgb8(imgbuf).save(fout, image::PNG)?;
        }
        ImageColor::RgbA => {
            let imgbuf: image::ImageBuffer<image::Rgba<_>, Vec<_>> =
                image::ImageBuffer::from_raw(width, height, data.to_owned())
                    .ok_or(errors::EncodingError)?;
            image::ImageRgba8(imgbuf).save(fout, image::PNG)?;
        }
    };
    Ok(())
}

fn decode(options: options::Options) -> Result<(), Error> {
    let image = match options.input_file {
        options::InputFile::Stdin => image::load_from_memory(&get_stdin()?)?,
        options::InputFile::File(path) => image::open(path)?,
    };
    let pixels = image.raw_pixels();
    println!("{}", String::from_utf8_lossy(&pixels));
    Ok(())
}

fn main() {
    let options = match options::get_options() {
        Ok(options) => options,
        Err(e) => {
            eprintln!("Invalid aguments");
            eprintln!("{:#?}", e);
            std::process::exit(1);
        }
    };

    let result = match options.mode {
        options::Mode::Encode => encode(options),
        options::Mode::Decode => decode(options),
    };

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
    };
}
