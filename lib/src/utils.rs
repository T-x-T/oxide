use std::{error::Error, io::{Read, Write}, net::TcpStream};

pub fn read_packet(mut stream: &TcpStream) -> crate::Packet {
  let mut packet_length_bits: Vec<u8> = Vec::new();
  loop {
    let buf: &mut [u8] = &mut [0];
    stream.read_exact(buf).unwrap();
    packet_length_bits.push(buf[0]);

    if buf[0] & 0x80 == 0 {
      break;
    }
  }
  let mut raw_packet = packet_length_bits.clone();

  let packet_length = crate::deserialize::varint(&mut packet_length_bits).unwrap();
  let mut packet: Vec<u8> = vec![0; packet_length as usize];
  stream.read_exact(&mut packet).unwrap();
  raw_packet.append(&mut packet.clone());

  let packet_id = crate::deserialize::varint(&mut packet).unwrap();

  return crate::Packet {
    id: packet_id as u8,
    length: packet_length as u32,
    data: packet,
    raw_data: raw_packet,
  };
}

pub fn send_packet(mut stream: &TcpStream, packet_id: u8, mut data: Vec<u8>) -> Result<(), Box<dyn Error>> {
  let mut serialized_id: Vec<u8> = crate::serialize::varint(packet_id as i32);
  let mut packet: Vec<u8> = crate::serialize::varint((data.len() + serialized_id.len()) as i32);
  packet.append(&mut serialized_id);
  packet.append(&mut data);

  stream.write_all(packet.as_slice())?;
  stream.flush()?;

  return Ok(());
}

pub fn u128_to_uuid_without_dashes(input: u128) -> String {
	return format!("{input:x}").to_string();
}

pub fn u128_to_uuid_with_dashes(input: u128) -> String {
	let short_form = u128_to_uuid_without_dashes(input);
	let mut long_form = short_form[0..=7].to_string();
	long_form.push('-');
	long_form.push_str(&short_form[8..=11]);
	long_form.push('-');
	long_form.push_str(&short_form[12..=15]);
	long_form.push('-');
	long_form.push_str(&short_form[16..=19]);
	long_form.push('-');
	long_form.push_str(&short_form[20..]);
	return long_form;
}

#[cfg(test)]
mod test {
  use super::*;

	#[test]
	fn works_for_me_without_dashes() {
		let res = u128_to_uuid_without_dashes(290780920670370370148908686767547353505);
		assert_eq!(res, "dac25e44d1024f3b819978ed62d209a1".to_string());
	}

	#[test]
	fn works_for_me_with_dashes() {
		let res = u128_to_uuid_with_dashes(290780920670370370148908686767547353505);
		assert_eq!(res, "dac25e44-d102-4f3b-8199-78ed62d209a1".to_string());
	}
}
