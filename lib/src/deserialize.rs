use std::error::Error;
use crate::CustomError;
use crate::nbt::NbtTag;

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

pub fn unsigned_short(data: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
  data.reverse();
  let first_byte = data.pop().unwrap();
  let second_byte = data.pop().unwrap();
  data.reverse();

  let output: u16 = (first_byte as u16 * 256 as u16) + second_byte as u16;

  return Ok(output);
}

pub fn short(data: &mut Vec<u8>) -> Result<i16, Box<dyn Error>> {
  let output: i16 = i16::from_be_bytes(data[..2].try_into().unwrap());
  data.drain(0..2);
  return Ok(output);
}

pub fn int(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  let output: i32 = i32::from_be_bytes(data[..4].try_into().unwrap());
  data.drain(0..4);
  return Ok(output);
}

pub fn long(data: &mut Vec<u8>) -> Result<i64, Box<dyn Error>> {
  let output: i64 = i64::from_be_bytes(data[..8].try_into().unwrap());
  data.drain(0..8);
  return Ok(output);
}

pub fn double(data: &mut Vec<u8>) -> Result<f64, Box<dyn Error>> {
  let output: f64 = f64::from_be_bytes(data[..8].try_into().unwrap());
  data.drain(0..8);
  return Ok(output);
}

pub fn float(data: &mut Vec<u8>) -> Result<f32, Box<dyn Error>> {
  let output: f32 = f32::from_be_bytes(data[..4].try_into().unwrap());
  data.drain(0..4);
  return Ok(output);
}

pub fn uuid(data: &mut Vec<u8>) -> Result<u128, Box<dyn Error>> {
  let output: u128 = u128::from_be_bytes(data[..16].try_into().unwrap());
  data.drain(0..16);
  return Ok(output);
}

pub fn string(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  let length = varint(data).unwrap();
  let raw_string: &[u8] = &data.clone()[..length as usize];
  data.drain(..length as usize);

  return Ok(String::from_utf8(raw_string.to_vec())?);
}

const SEGMENT_BITS: u8 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000; 

pub fn varint(data: &mut Vec<u8>) -> Result<i32, Box<dyn Error>> {
  if data.is_empty() {
    return Err(Box::new(crate::CustomError::ParseInvalidValue));
  }

  let mut value: i32 = 0;
  let mut position = 0;
  let mut current_byte: u8;

  loop {
    current_byte = data.remove(0);
    value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

    if (current_byte & CONTINUE_BIT) == 0 {
      break;
    }

    position += 7;

    if position >= 32 {
      return Err(Box::new(CustomError::ParseVarIntTooBig));
    }
  }
  return Ok(value);
}

pub fn nbt(data: &mut Vec<u8>) -> Result<NbtTag, Box<dyn Error>> {
  return nbt_tag_compound(data, false, true);
}

fn nbt_byte(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  let output = NbtTag::Byte(description, data.pop().unwrap());
  data.reverse();

  return Ok(output);
}

fn nbt_short(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::Short(description, short(data)?);

  return Ok(output);
}

fn nbt_int(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::Int(description, int(data)?);

  return Ok(output);
}

fn nbt_long(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::Long(description, long(data)?);

  return Ok(output);
}

fn nbt_float(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::Float(description, float(data)?);

  return Ok(output);
}

fn nbt_double(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::Double(description, double(data)?);

  return Ok(output);
}

fn nbt_byte_array(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  let output = NbtTag::ByteArray(description, nbt_byte_array_value(&mut data)?);

  return Ok(output);
}

fn nbt_byte_array_value(mut data: &mut Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
  let len = int(&mut data)?;
  let mut bytes: Vec<u8> = Vec::new();
  data.reverse();
  for _ in 0..len {
    bytes.push(data.pop().unwrap());
  }
  data.reverse();
  return Ok(bytes);
}

fn nbt_string(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };
  
  data.reverse();
  let output = NbtTag::String(description, nbt_string_value(&mut data)?);

  return Ok(output);
}

pub fn nbt_string_value(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  data.reverse();

  let mut bytes: [u8; 2] = [0, 0];
  bytes[0] = data.pop().unwrap();
  bytes[1] = data.pop().unwrap();
  let len = i16::from_be_bytes(bytes);
  
  data.reverse();
  let raw_string: &[u8] = &data.clone()[..len as usize];
  data.drain(..len as usize);
  let string = String::from_utf8(raw_string.to_vec())?;

  return Ok(string);
}

fn nbt_list(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };
  
  let id = data.pop().unwrap();
  let len = int(&mut data)?;

  let output: NbtTag = match id {
    0x01 => {
      let mut list: Vec<NbtTag> = Vec::new();
      for _ in 0..len {
        list.push(NbtTag::Byte(None, data.pop().unwrap()));
      }

      NbtTag::List(description, list)
    },
    0x02 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::Short(None, short(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x03 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::Int(None, int(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x04 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::Long(None, long(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x05 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::Float(None, float(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x06 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::Double(None, double(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x07 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::ByteArray(None, nbt_byte_array_value(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x08 => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::String(None, nbt_string_value(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x09 => {
      todo!("this isnt implemented yet lol");
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::String(None, nbt_string_value(&mut data)?));
      }
      data.reverse();
      
      NbtTag::List(description, list)
    },
    0x0a => {
      todo!("this isnt implemented yet lol");
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::String(None, nbt_string_value(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x0b => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::IntArray(None, nbt_int_array_value(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    0x0c => {
      let mut list: Vec<NbtTag> = Vec::new();
      data.reverse();
      for _ in 0..len {
        list.push(NbtTag::LongArray(None, nbt_long_array_value(&mut data)?));
      }
      data.reverse();

      NbtTag::List(description, list)
    },
    _ => {
      return Err(Box::new(CustomError::ParseInvalidValue));
    }
  };
  
  data.reverse();

  return Ok(output);
}

fn nbt_tag_compound(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.pop().unwrap();

    match id {
      0x00 => break,
      0x01 => {
        println!("decode byte");
        tags.push(NbtTag::Byte(Some(nbt_string_value(&mut data)?), data.pop().unwrap()));
      },
      0x02 => {
        println!("decode short");
        data.reverse();
        tags.push(NbtTag::Short(Some(nbt_string_value(&mut data)?), short(&mut data)?));
        data.reverse();
      },
      0x03 => {
        println!("decode int");
        data.reverse();
        tags.push(NbtTag::Int(Some(nbt_string_value(&mut data)?), int(&mut data)?));
        data.reverse();
      },
      0x04 => {
        println!("decode long");
        data.reverse();
        tags.push(NbtTag::Long(Some(nbt_string_value(&mut data)?), long(&mut data)?));
        data.reverse();
      },
      0x05 => {
        println!("decode float");
        data.reverse();
        tags.push(NbtTag::Float(Some(nbt_string_value(&mut data)?), float(&mut data)?));
        data.reverse();
      },
      0x06 => {
        println!("decode double");
        data.reverse();
        tags.push(NbtTag::Double(Some(nbt_string_value(&mut data)?), double(&mut data)?));
        data.reverse();
      },
      0x07 => {
        println!("decode byte array");
        data.reverse();
        tags.push(NbtTag::ByteArray(Some(nbt_string_value(&mut data)?), nbt_byte_array_value(&mut data)?));
        data.reverse();
      },
      0x08 => {
        println!("decode string");
        data.reverse();
        tags.push(NbtTag::String(Some(nbt_string_value(&mut data)?), nbt_string_value(&mut data)?));
        data.reverse();
      },
      0x09 => {
        println!("decode list");
        todo!("this isnt implemented yet lol");
        data.reverse();
        tags.push(NbtTag::String(Some(nbt_string_value(&mut data)?), nbt_string_value(&mut data)?));
        data.reverse();
      },
      0x0a => {
        println!("decode tag compound");
        todo!("this isnt implemented yet lol");
        data.reverse();
        tags.push(NbtTag::String(Some(nbt_string_value(&mut data)?), nbt_string_value(&mut data)?));
        data.reverse();
      },
      0x0b => {
        println!("decode int array");
        data.reverse();
        tags.push(NbtTag::IntArray(Some(nbt_string_value(&mut data)?), nbt_int_array_value(&mut data)?));
        data.reverse();
      },
      0x0c => {
        println!("decode long array");
        data.reverse();
        tags.push(NbtTag::LongArray(Some(nbt_string_value(&mut data)?), nbt_long_array_value(&mut data)?));
        data.reverse();
      },
      _ => {
        println!("decode invalid id: {id}");
        return Err(Box::new(CustomError::ParseInvalidValue));
      }
    };
  }

  let output = NbtTag::TagCompound(description, tags);

  data.reverse();

  return Ok(output);
}

fn nbt_int_array(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  return Ok(NbtTag::IntArray(description, nbt_int_array_value(&mut data)?));
}

fn nbt_int_array_value(data: &mut Vec<u8>) -> Result<Vec<i32>, Box<dyn Error>> {
  let length = int(data)?;

  let mut arr: Vec<i32> = Vec::new();
  for _ in 0..length {
    arr.push(int(data)?);
  }

  return Ok(arr);
}

fn nbt_long_array(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  data.reverse();
  if has_id {
    data.pop();
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  data.reverse();
  return Ok(NbtTag::LongArray(description, nbt_long_array_value(&mut data)?));
}

fn nbt_long_array_value(data: &mut Vec<u8>) -> Result<Vec<i64>, Box<dyn Error>> {
  let length = int(data)?;

  let mut arr: Vec<i64> = Vec::new();
  for _ in 0..length {
    arr.push(long(data)?);
  }

  return Ok(arr);
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn nbt_short() {
    let number: i16 = 28591;
    let mut serialized = crate::serialize::nbt(NbtTag::Short(None, number));
    let deserialized = super::nbt_short(&mut serialized, false, true).unwrap();
    let deserialized_number = match deserialized {
      NbtTag::Short(_, x) => {
        x
      },
      _ => panic!("a"),
    };

    assert_eq!(deserialized_number, number);
  }

  #[test]
  fn nbt_short_negative() {
    let number: i16 = -28591;
    let mut serialized = crate::serialize::nbt(NbtTag::Short(None, number));
    let deserialized = super::nbt_short(&mut serialized, false, true).unwrap();
    let deserialized_number = match deserialized {
      NbtTag::Short(_, x) => {
        x
      },
      _ => panic!("a"),
    };

    assert_eq!(deserialized_number, number);
  }

  #[test]
  fn varint_works_small_number() {
    let res = varint(&mut vec![0x01]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
  }

  #[test]
  fn varint_works_large_number() {
    let res = varint(&mut vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2147483647);
  }

  #[test]
  fn varint_works_zero() {
    let res = varint(&mut vec![0x00]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0);
  }

  #[test]
  fn varint_works_negative() {
    let res = varint(&mut vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -1);
  }

  #[test]
  fn varint_works_medium_number() {
    let res = varint(&mut vec![0xf4, 0xd0, 0x01]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 26740);
  }
}