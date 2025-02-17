pub mod handlers;
pub mod senders;



#[allow(unused)]
fn print_binary(packet: &Vec<u8>) {
  packet.iter().for_each(|x| println!("{:2x?} {:0>8b} {}", x, x, x));
}


#[cfg(test)]
mod test {
  use lib::deserialize;
	use lib::serialize;

  #[test]
  fn varint_parse_serialize_single_byte() {
    let val = 120;
    assert_eq!(val, deserialize::varint(&mut serialize::varint(val)).unwrap());
  }

  #[test]
  fn varint_parse_serialize_four_byte() {
    let val = 1073741820;
    assert_eq!(val, deserialize::varint(&mut serialize::varint(val)).unwrap());
  }

  #[test]
  fn varint_parse_serialize_single_byte_negative() {
    let val = -1;
    assert_eq!(val, deserialize::varint(&mut serialize::varint(val)).unwrap());
  }

  #[test]
  fn varint_parse_serialize_four_byte_negative() {
    let val = -1073741820;
    assert_eq!(val, deserialize::varint(&mut serialize::varint(val)).unwrap());
  }

  #[test]
  fn varint_parse_serialize_zero() {
    let val = 0;
    assert_eq!(val, deserialize::varint(&mut serialize::varint(val)).unwrap());
  }
}