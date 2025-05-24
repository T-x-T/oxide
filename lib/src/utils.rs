use std::{error::Error, io::{Read, Write}, net::TcpStream};

pub fn read_packet(mut stream: &TcpStream) -> crate::Packet {
  let mut packet_length_bits: Vec<u8> = Vec::new();
  loop {
    let buf: &mut [u8] = &mut [0];
    stream.read(buf).unwrap();
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

  stream.write(packet.as_slice())?;
  stream.flush()?;

  return Ok(());
}
