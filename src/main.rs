extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate image;

use failure::Error;

mod errors;
mod options;

fn main() {
    let options = options::get_options();
}
