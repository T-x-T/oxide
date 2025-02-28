use crate::nbt::NbtTag;

const SEGMENT_BITS: u32 = 0b0111_1111;
const CONTINUE_BIT: u8 = 0b1000_0000; 

pub fn varint(value: i32) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  let mut uvalue = value as u32;
  loop {
    let mut byte = (uvalue & SEGMENT_BITS) as u8;
    uvalue >>= 7;
    
    if uvalue != 0 {
      byte |= CONTINUE_BIT;
    }
    
    output.push(byte);
    
    if uvalue == 0 {
      break;
    }
  }

  return output;
}

pub fn float(input: f32) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn double(input: f64) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn short(input: i16) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn int(input: i32) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn long(input: i64) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn string(input: &str) -> Vec<u8> {
  let mut output: Vec<u8> = varint(input.len() as i32);

  output.append(&mut input.as_bytes().to_vec());

  return output;
}

pub fn uuid(input: &u128) -> Vec<u8> {
  return input.to_be_bytes().to_vec();
}

pub fn bool(input: &bool) -> Vec<u8> {
  return match input {
    true => vec![1],
    false => vec![0],
  }
}

pub fn nbt(input: NbtTag) -> Vec<u8> {
  let mut nbt = nbt_tag_compound(None, vec![input], false);
  nbt.pop(); //Otherwise we have one 0x00 byte too much at the end
  return nbt;
}

fn nbt_byte(description: Option<&str>, payload: u8, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x01);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

    output.push(payload);

  return output;
}

fn nbt_short(description: Option<&str>, payload: i16, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x02);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_int(description: Option<&str>, payload: i32, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x03);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_long(description: Option<&str>, payload: i64, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x04);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_float(description: Option<&str>, payload: f32, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x05);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_double(description: Option<&str>, payload: f64, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x06);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut payload.to_be_bytes().into());

  return output;
}

fn nbt_byte_array(description: Option<&str>, payload: &[u8], include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x07);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());

  output.append(&mut payload.to_vec());

  return output;
}

fn nbt_string(description: Option<&str>, payload: &str, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x08);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  output.append(&mut nbt_short(None, payload.len() as i16, false));
  output.append(&mut payload.as_bytes().to_vec());

  return output;
}

fn nbt_list(description: Option<&str>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x09);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  if payload.len() == 0 {
    return output;
  }

  let length: i32 = payload.len() as i32;

  match payload[0] {
    NbtTag::Byte(_, _) => {
      output.push(0x01);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte(None, match x {
        NbtTag::Byte(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Short(_, _) => {
      output.push(0x02);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_short(None, match x {
        NbtTag::Short(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Int(_, _) => {
      output.push(0x03);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, match x {
        NbtTag::Int(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Long(_, _) => {
      output.push(0x04);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, match x {
        NbtTag::Long(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Float(_, _) => {
      output.push(0x05);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_float(None, match x {
        NbtTag::Float(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::Double(_, _) => {
      output.push(0x06);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_double(None, match x {
        NbtTag::Double(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::ByteArray(_, _) => {
      output.push(0x07);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_byte_array(None, match x {
        NbtTag::ByteArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::String(_, _) => {
      output.push(0x08);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_string(None, match x {
        NbtTag::String(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::List(_, _) => {
      output.push(0x09);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_list(None, match x {
        NbtTag::List(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::TagCompound(_, _) => {
      output.push(0x0a);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_tag_compound(None, match x {
        NbtTag::TagCompound(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::IntArray(_, _) => {
      output.push(0x0b);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_int_array(None, match x {
        NbtTag::IntArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
    NbtTag::LongArray(_, _) => {
      output.push(0x0c);
      output.append(&mut length.to_be_bytes().into());
      payload.into_iter().for_each(|x| output.append(&mut nbt_long_array(None, match x {
        NbtTag::LongArray(_, x) => x,
        _ => panic!("impossible to reach"),
      }, false)));
    },
  };

  return output;
}

fn nbt_tag_compound(description: Option<&str>, payload: Vec<NbtTag>, include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0a);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  if payload.len() == 0 {
    return output;
  }

  payload.into_iter().for_each(|x| {
    match x {
      NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p, true)),
      NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p, true)),
      NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p, true)),
      NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p, true)),
      NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p, true)),
      NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p, true)),
      NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p, true)),
      NbtTag::String(d, p) => output.append(&mut nbt_string(d, p, true)),
      NbtTag::List(d, p) => output.append(&mut nbt_list(d, p, true)),
      NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p, true)),
      NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p, true)),
      NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p, true)),
    };
  });

  output.push(0x00);
  return output;
}

fn nbt_int_array(description: Option<&str>, payload: &[i32], include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0b);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_int(None, *x, false)));

  return output;
}

fn nbt_long_array(description: Option<&str>, payload: &[i64], include_id: bool) -> Vec<u8> {
  let mut output: Vec<u8> = Vec::new();

  if include_id {
    output.push(0x0c);
  }

  if description.is_some() {
    output.append(&mut nbt_string(None, description.unwrap(), false));
  }

  let length: i32 = payload.len() as i32;
  output.append(&mut length.to_be_bytes().into());

  payload.into_iter().for_each(|x| output.append(&mut nbt_long(None, *x, false)));

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn varint_works_small_number() {
    let res = varint(1);
    assert_eq!(res, vec![0x01]);
  }

  #[test]
  fn varint_works_large_number() {
    let res = varint(2147483647);
    assert_eq!(res, vec![0xff, 0xff, 0xff, 0xff, 0x07]);
  }

  #[test]
  fn varint_works_zero() {
    let res = varint(0);
    assert_eq!(res, vec![0x00]);
  }

  #[test]
  fn varint_works_negative() {
    let res = varint(-1);
    assert_eq!(res, vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
  }

  #[test]
  fn varint_works_medium_number() {
    let res = varint(26740);
    assert_eq!(res, vec![0xf4, 0xd0, 0x01]);
  }

  #[test]
  #[ignore]
  fn test() {
    use std::fs::write;

    let test_nbt = NbtTag::TagCompound(Some("Level"), vec![
        NbtTag::Long(Some("longTest"), 9223372036854775807),
        NbtTag::Short(Some("shortTest"), 32767),
        NbtTag::String(Some("stringTest"), "HELLO WORLD THIS IS A TEST STRING ÅÄÖ!"),
        NbtTag::Float(Some("floatTest"), 0.4982314705848694),
        NbtTag::Int(Some("intTest"), 2147483647),
        NbtTag::TagCompound(Some("nested compound test"), vec![
          NbtTag::TagCompound(Some("ham"), vec![
            NbtTag::String(Some("name"), "Hampus"),
            NbtTag::Float(Some("value"), 0.75)
          ]),
          NbtTag::TagCompound(Some("egg"), vec![
            NbtTag::String(Some("name"), "Eggbert"),
            NbtTag::Float(Some("value"), 0.5)
          ])
        ]),
        NbtTag::List(Some("listTest (long)"), vec![
          NbtTag::Long(None, 11),
          NbtTag::Long(None, 12),
          NbtTag::Long(None, 13),
          NbtTag::Long(None, 14),
          NbtTag::Long(None, 15)
        ]),
        NbtTag::List(Some("listTest (compound)"), vec![
          NbtTag::TagCompound(None, vec![
            NbtTag::String(Some("name"), "Compound tag #0"),
            NbtTag::Long(Some("created-on"), 1264099775885)
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::String(Some("name"), "Compound tag #1"),
            NbtTag::Long(Some("created-on"), 1264099775885)
          ])
        ]),
        NbtTag::Byte(Some("byteTest"), 127),
        NbtTag::ByteArray(Some("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))"), &[0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48,0,62,34,16,8,10,22,44,76,18,70,32,4,86,78,80,92,14,46,88,40,2,74,56,48,50,62,84,16,58,10,72,44,26,18,20,32,54,86,28,80,42,14,96,88,90,2,24,56,98,50,12,84,66,58,60,72,94,26,68,20,82,54,36,28,30,42,64,96,38,90,52,24,6,98,0,12,34,66,8,60,22,94,76,68,70,82,4,36,78,30,92,64,46,38,40,52,74,6,48]),
        NbtTag::Double(Some("doubleTest"), 0.4931287132182315),
      ]);

    write("/tmp/my_bigtest.nbt", nbt(test_nbt.clone())).unwrap();
  }
}