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
    _ => Err(Box::new(CustomError::DeserializeInvalidBoolean(value))),
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
    return Err(Box::new(crate::CustomError::InputEmpty));
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
  //println!("deserialize nbt:\n\n{data:?}\n\n\n\n");
  return nbt_tag_compound(data, false, true);
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

pub fn nbt_string_value(data: &mut Vec<u8>) -> Result<String, Box<dyn Error>> {
  let mut bytes: [u8; 2] = [0, 0];
  bytes[0] = data.remove(0);
  bytes[1] = data.remove(0);
  let len = i16::from_be_bytes(bytes);
  
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
    x => {
      return Err(Box::new(CustomError::InvalidNbtTag(x)));
    }
  };
  
  data.reverse();

  return Ok(output);
}

fn nbt_tag_compound(mut data: &mut Vec<u8>, has_description: bool, has_id: bool) -> Result<NbtTag, Box<dyn Error>> {
  if has_id {
    data.remove(0);
  }

  let description: Option<String> = if has_description {
    Some(nbt_string_value(&mut data)?)
  } else {
    None
  };

  let mut tags: Vec<NbtTag> = Vec::new();

  loop {
    let id = data.remove(0);

    match id {
      0x00 => break,
      0x01 => {
        tags.push(NbtTag::Byte(Some(nbt_string_value(&mut data)?), data.remove(0)));
      },
      0x02 => {
        tags.push(NbtTag::Short(Some(nbt_string_value(&mut data)?), short(&mut data)?));
      },
      0x03 => {
        tags.push(NbtTag::Int(Some(nbt_string_value(&mut data)?), int(&mut data)?));
      },
      0x04 => {
        tags.push(NbtTag::Long(Some(nbt_string_value(&mut data)?), long(&mut data)?));
      },
      0x05 => {
        tags.push(NbtTag::Float(Some(nbt_string_value(&mut data)?), float(&mut data)?));
      },
      0x06 => {
        tags.push(NbtTag::Double(Some(nbt_string_value(&mut data)?), double(&mut data)?));
      },
      0x07 => {
        tags.push(NbtTag::ByteArray(Some(nbt_string_value(&mut data)?), nbt_byte_array_value(&mut data)?));
      },
      0x08 => {
        tags.push(NbtTag::String(Some(nbt_string_value(&mut data)?), nbt_string_value(&mut data)?));
      },
      0x09 => {
        tags.push(nbt_list(&mut data, true, false)?);
      },
      0x0a => {
        tags.push(nbt_tag_compound(&mut data, true, false)?);
      },
      0x0b => {
        tags.push(NbtTag::IntArray(Some(nbt_string_value(&mut data)?), nbt_int_array_value(&mut data)?));
      },
      0x0c => {
        tags.push(NbtTag::LongArray(Some(nbt_string_value(&mut data)?), nbt_long_array_value(&mut data)?));
      },
      x => {
        return Err(Box::new(CustomError::InvalidNbtTag(x)));
      }
    };
  }
  
  let output = NbtTag::TagCompound(description, tags);

  return Ok(output);
}

fn nbt_int_array_value(data: &mut Vec<u8>) -> Result<Vec<i32>, Box<dyn Error>> {
  let length = int(data)?;

  let mut arr: Vec<i32> = Vec::new();
  for _ in 0..length {
    arr.push(int(data)?);
  }

  return Ok(arr);
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

  #[test]
  fn varint_works_256() {
    let res = varint(&mut vec![128, 2]);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 256);
  }

  #[test]
  fn nbt_mobspawner() {
    let nbt_parsed = NbtTag::TagCompound(None, vec![
      NbtTag::Short(Some("MaxNearbyEntities".to_string()), 6),
      NbtTag::Short(Some("RequiredPlayerRange".to_string()), 16),
      NbtTag::Short(Some("SpawnCount".to_string()), 4),
      NbtTag::TagCompound(Some("SpawnData".to_string()), vec![
        NbtTag::TagCompound(Some("entity".to_string()), vec![
          NbtTag::String(Some("id".to_string()), "minecraft:spider".to_string()),
        ]),
      ]),
      NbtTag::Short(Some("MaxSpawnDelay".to_string()), 800),
      NbtTag::Short(Some("SpawnRange".to_string()), 4),
      NbtTag::Short(Some("Delay".to_string()), 20),
      NbtTag::Short(Some("MinSpawnDelay".to_string()), 200),
    ]);

    let mut nbt_bytes: Vec<u8> = vec![10,2,0,17,77,97,120,78,101,97,114,98,121,69,110,116,105,116,105,101,115,0,6,2,0,19,82,101,113,117,105,114,101,100,80,108,97,121,101,114,82,97,110,103,101,0,16,2,0,10,83,112,97,119,110,67,111,117,110,116,0,4,10,0,9,83,112,97,119,110,68,97,116,97,10,0,6,101,110,116,105,116,121,8,0,2,105,100,0,16,109,105,110,101,99,114,97,102,116,58,115,112,105,100,101,114,0,0,2,0,13,77,97,120,83,112,97,119,110,68,101,108,97,121,3,32,2,0,10,83,112,97,119,110,82,97,110,103,101,0,4,2,0,5,68,101,108,97,121,0,20,2,0,13,77,105,110,83,112,97,119,110,68,101,108,97,121,0,200,0];
  
    assert_eq!(nbt(&mut nbt_bytes).unwrap(), nbt_parsed);
  }
}