use std::{error::Error, fmt::{Display, Formatter, Result}};

pub mod serialize;
pub mod deserialize;
pub mod nbt;

#[derive(Debug, Clone)]
enum CustomError {
  ParseVarIntTooBig,
  ParseInvalidValue,
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return match self {
      CustomError::ParseVarIntTooBig => write!(f, "Tried to parse varint that was too large"),
      CustomError::ParseInvalidValue => write!(f, "Tried to parse invalid value"),
    }
  }
}

impl Error for CustomError {

}
