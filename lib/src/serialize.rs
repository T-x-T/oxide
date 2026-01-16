use crate::types::*;

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

pub fn boolean(input: bool) -> Vec<u8> {
	if input {
		return vec![0x01];
	} else {
		return vec![0x00];
	}
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

pub fn unsigned_short(input: u16) -> Vec<u8> {
	return input.to_be_bytes().to_vec();
}

pub fn int(input: i32) -> Vec<u8> {
	return input.to_be_bytes().to_vec();
}

pub fn long(input: i64) -> Vec<u8> {
	return input.to_be_bytes().to_vec();
}

pub fn unsigned_long(input: u64) -> Vec<u8> {
	return input.to_be_bytes().to_vec();
}

pub fn string(input: &str) -> Vec<u8> {
	let mut output: Vec<u8> = varint(input.len() as i32);

	output.append(&mut input.as_bytes().to_vec());

	return output;
}

pub fn bitset(input: &Vec<u64>) -> Vec<u8> {
	let mut output: Vec<u8> = varint(input.len() as i32);
	for x in input {
		output.append(&mut unsigned_long(*x));
	}
	return output;
}

pub fn position(input: &crate::types::position::BlockPosition) -> Vec<u8> {
	return unsigned_long(((input.x as u64 & 0x3FFFFFF) << 38) | ((input.z as u64 & 0x3FFFFFF) << 12) | (input.y as u64 & 0xFFF));
}

pub fn uuid(input: &u128) -> Vec<u8> {
	return input.to_be_bytes().to_vec();
}

pub fn prefixed_array(mut data: Vec<u8>, len: i32) -> Vec<u8> {
	let mut output: Vec<u8> = varint(len);
	output.append(&mut data);

	return output;
}

pub fn nbt_network(input: NbtTag) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	match input {
		NbtTag::Root(p) => {
			output.push(0x0a);

			p.into_iter().for_each(|x| {
				match x {
					NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
					NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
					NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
					NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
					NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
					NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
					NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
					NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
					NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
					NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
					NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
					NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
					NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
				};
			});

			output.push(0x00);
		}
		_ => panic!("root node must be a root tag"),
	}

	return output;
}

pub fn nbt_disk(input: NbtTag) -> Vec<u8> {
	match input {
		NbtTag::Root(p) => {
			return nbt_tag_compound("".to_string(), p);
		}
		_ => panic!("root node must be a tag compound"),
	}
}

fn nbt_byte(description: String, payload: u8) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x01);
	output.append(&mut nbt_string_list(description));

	output.push(payload);

	return output;
}

fn nbt_byte_list(payload: u8) -> Vec<u8> {
	return vec![payload];
}

fn nbt_short(description: String, payload: i16) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x02);
	output.append(&mut nbt_string_list(description));

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_short_list(payload: i16) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_int(description: String, payload: i32) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x03);
	output.append(&mut nbt_string_list(description));

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_int_list(payload: i32) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_long(description: String, payload: i64) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x04);
	output.append(&mut nbt_string_list(description));

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_long_list(payload: i64) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_float(description: String, payload: f32) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x05);
	output.append(&mut nbt_string_list(description));

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_float_list(payload: f32) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_double(description: String, payload: f64) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x06);
	output.append(&mut nbt_string_list(description));

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_double_list(payload: f64) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut payload.to_be_bytes().into());

	return output;
}

fn nbt_byte_array(description: String, payload: Vec<u8>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x07);
	output.append(&mut nbt_string_list(description));

	output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());
	output.append(&mut payload.to_vec());

	return output;
}

fn nbt_byte_array_list(payload: Vec<u8>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut (payload.len() as i32).to_be_bytes().to_vec());
	output.append(&mut payload.to_vec());

	return output;
}

fn nbt_string(description: String, payload: String) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x08);
	output.append(&mut nbt_string_list(description));

	output.append(&mut nbt_short_list(payload.len() as i16));
	output.append(&mut payload.as_bytes().to_vec());

	return output;
}

fn nbt_string_list(payload: String) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	output.append(&mut nbt_short_list(payload.len() as i16));
	output.append(&mut payload.as_bytes().to_vec());

	return output;
}

fn nbt_list(description: String, payload: Vec<NbtListTag>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x09);
	output.append(&mut nbt_string_list(description));

	output.append(&mut actual_nbt_list(payload));

	return output;
}

fn nbt_list_list(payload: Vec<NbtListTag>) -> Vec<u8> {
	return actual_nbt_list(payload);
}

fn actual_nbt_list(payload: Vec<NbtListTag>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	if payload.is_empty() {
		output.append(&mut vec![0; 5]);
		return output;
	}

	let length: i32 = payload.len() as i32;
	match payload[0] {
		NbtListTag::Byte(_) => {
			output.push(0x01);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_byte_list(match x {
					NbtListTag::Byte(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::Short(_) => {
			output.push(0x02);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_short_list(match x {
					NbtListTag::Short(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::Int(_) => {
			output.push(0x03);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_int_list(match x {
					NbtListTag::Int(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::Long(_) => {
			output.push(0x04);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_long_list(match x {
					NbtListTag::Long(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::Float(_) => {
			output.push(0x05);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_float_list(match x {
					NbtListTag::Float(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::Double(_) => {
			output.push(0x06);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_double_list(match x {
					NbtListTag::Double(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::ByteArray(_) => {
			output.push(0x07);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_byte_array_list(match x {
					NbtListTag::ByteArray(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::String(_) => {
			output.push(0x08);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_string_list(match x {
					NbtListTag::String(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::List(_) => {
			output.push(0x09);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_list_list(match x {
					NbtListTag::List(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::TagCompound(_) => {
			output.push(0x0a);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_tag_compound_list(match x {
					NbtListTag::TagCompound(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::IntArray(_) => {
			output.push(0x0b);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_int_array_list(match x {
					NbtListTag::IntArray(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
		NbtListTag::LongArray(_) => {
			output.push(0x0c);
			output.append(&mut length.to_be_bytes().into());
			payload.into_iter().for_each(|x| {
				output.append(&mut nbt_long_array_list(match x {
					NbtListTag::LongArray(x) => x,
					_ => panic!("impossible to reach"),
				}))
			});
		}
	};

	return output;
}

fn nbt_tag_compound(description: String, payload: Vec<NbtTag>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x0a);
	output.append(&mut nbt_string_list(description));

	if payload.is_empty() {
		output.push(0x00);
		return output;
	}

	payload.into_iter().for_each(|x| {
		match x {
			NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
			NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
			NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
			NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
			NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
			NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
			NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
			NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
			NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
			NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
			NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
			NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
			NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
		};
	});

	output.push(0x00);
	return output;
}

fn nbt_tag_compound_list(payload: Vec<NbtTag>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	if payload.is_empty() {
		output.push(0x00);
		return output;
	}

	payload.into_iter().for_each(|x| {
		match x {
			NbtTag::Byte(d, p) => output.append(&mut nbt_byte(d, p)),
			NbtTag::Short(d, p) => output.append(&mut nbt_short(d, p)),
			NbtTag::Int(d, p) => output.append(&mut nbt_int(d, p)),
			NbtTag::Long(d, p) => output.append(&mut nbt_long(d, p)),
			NbtTag::Float(d, p) => output.append(&mut nbt_float(d, p)),
			NbtTag::Double(d, p) => output.append(&mut nbt_double(d, p)),
			NbtTag::ByteArray(d, p) => output.append(&mut nbt_byte_array(d, p)),
			NbtTag::String(d, p) => output.append(&mut nbt_string(d, p)),
			NbtTag::List(d, p) => output.append(&mut nbt_list(d, p)),
			NbtTag::TagCompound(d, p) => output.append(&mut nbt_tag_compound(d, p)),
			NbtTag::Root(_) => panic!("Root tag cannot be in a TagCompound!!"),
			NbtTag::IntArray(d, p) => output.append(&mut nbt_int_array(d, p)),
			NbtTag::LongArray(d, p) => output.append(&mut nbt_long_array(d, p)),
		};
	});

	output.push(0x00);
	return output;
}

fn nbt_int_array(description: String, payload: Vec<i32>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x0b);
	output.append(&mut nbt_string_list(description));

	let length: i32 = payload.len() as i32;
	output.append(&mut length.to_be_bytes().into());

	payload.into_iter().for_each(|x| output.append(&mut nbt_int_list(x)));

	return output;
}

fn nbt_int_array_list(payload: Vec<i32>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	let length: i32 = payload.len() as i32;
	output.append(&mut length.to_be_bytes().into());

	payload.into_iter().for_each(|x| output.append(&mut nbt_int_list(x)));

	return output;
}

fn nbt_long_array(description: String, payload: Vec<i64>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();
	output.push(0x0c);
	output.append(&mut nbt_string_list(description));

	let length: i32 = payload.len() as i32;
	output.append(&mut length.to_be_bytes().into());

	payload.into_iter().for_each(|x| output.append(&mut nbt_long_list(x)));

	return output;
}

fn nbt_long_array_list(payload: Vec<i64>) -> Vec<u8> {
	let mut output: Vec<u8> = Vec::new();

	let length: i32 = payload.len() as i32;
	output.append(&mut length.to_be_bytes().into());

	payload.into_iter().for_each(|x| output.append(&mut nbt_long_list(x)));

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
}
