#[derive(Fail, Debug)]
#[fail(display = "Invalid arguments")]
pub struct OptionsError;

#[derive(Fail, Debug)]
#[fail(display = "Unknown error creating image")]
pub struct EncodingError;
