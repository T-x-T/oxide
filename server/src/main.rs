use std::fmt;
use std::error::Error;

mod server;
mod nbt;

fn main() {
  println!("Welcome to oxide!");
  server::java::initialize_server();
}


#[derive(Debug, Clone)]
enum CustomError {
  ParseVarIntTooBig,
  ParseInvalidValue,
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return match self {
      CustomError::ParseVarIntTooBig => write!(f, "Tried to parse varint that was too large"),
      CustomError::ParseInvalidValue => write!(f, "Tried to parse invalid value"),
    }
  }
}

impl Error for CustomError {

}