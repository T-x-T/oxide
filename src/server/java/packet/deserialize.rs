use std::error::Error;
use crate::CustomError;

#[allow(dead_code)]
pub fn boolean(data: &mut Vec<u8>) -> Result<bool, Box<dyn Error>> {
  data.reverse();
  let value = data.pop().unwrap();
  data.reverse();

  return match value {
    0x00 => Ok(false),
    0x01 => Ok(true),
    _ => Err(Box::new(CustomError::ParseInvalidValue))
  }
}

#[allow(dead_code)]
pub fn unsigned_short(data: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
  data.reverse();
  let first_byte = data.pop().unwrap();
  let second_byte = data.pop().unwrap();
  data.reverse();

  let output: u16 = (first_byte as u16 * 256 as u16) + second_byte as u16;

  return Ok(output);
}

#[allow(dead_code)]
pub fn long(data: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
  let output: i64 = i64::from_be_bytes(data[..8].try_into().unwrap());
  data.drain(0..7);
  return Ok(output);
}

#[allow(dead_code)]
pub fn double(data: &mut Vec<u8>) -> Result<f64, Box<dyn Error>> {
  let output: f64 = f64::from_be_bytes(data[..8].try_into().unwrap());
  data.drain(0..7);
  return Ok(output);
}

#[allow(dead_code)]
pub fn float(data: &mut Vec<u8>) -> Result<f32, Box<dyn Error>> {
  let output: f32 = f32::from_be_bytes(data[..4].try_into().unwrap());
  data.drain(0..3);
  return Ok(output);
}

#[allow(dead_code)]
pub fn uuid(data: &mut Vec<u8>) -> Result<u128, Box<dyn Error>> {
  let output: u128 = u128::from_be_bytes(data[..16].try_into().unwrap());
  data.drain(0..15);
  return Ok(output);
}

#[allow(dead_code)]
pub fn string(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  let length = varint(data).unwrap();
  let raw_string: &[u8] = &data.clone()[..length as usize];
  data.drain(..length as usize);

  return Ok(String::from_utf8(raw_string.to_vec())?);
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80; 

#[allow(dead_code)]
pub fn varint(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  let mut value: i32 = 0;
  let mut position = 0;
  let mut current_byte: u8;
  data.reverse();

  loop {
    current_byte = data.pop().unwrap_or(0);
    value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

    if (current_byte & CONTINUE_BIT) == 0 {
      break;
    }

    position += 7;

    if position >= 32 {
      return Err(Box::new(CustomError::ParseVarIntTooBig));
    }
  }

  data.reverse();

  return Ok(value);
}