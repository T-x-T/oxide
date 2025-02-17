use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

fn main() {
  println!("Starting the oxide proxy");
  
  let listener = TcpListener::bind("127.0.0.1:35565").unwrap();
  
  for stream in listener.incoming() {
    let mut server_read_stream = stream.unwrap();
    println!("New Connection from {}", server_read_stream.peer_addr().unwrap());

    let mut client_send_stream = TcpStream::connect("127.0.0.1:25565").unwrap();
    let mut client_read_stream = client_send_stream.try_clone().unwrap();
    let mut server_send_stream = server_read_stream.try_clone().unwrap();

    //Handle packets coming in on the server side
    std::thread::spawn(move || {
      loop {
        let mut peek_buf = [0; 1];
        
        match server_read_stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("server disconnected.");
            break;
          }
          Err(e) => {
            eprintln!("error reading from server: {}", e);
            break;
          }
          _ => {}
        }

        println!("server received a packet:");
        let server_packet = read_packet(&mut server_read_stream);
        send_packet(&mut client_send_stream, server_packet);
      }
    });
    println!("server listeneder spawned");
    
    //Handle packets coming in on the client side
    std::thread::spawn(move || {
      loop {
        let mut peek_buf = [0; 1];

        match client_read_stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {}", e);
            break;
          }
          _ => {}
        }
  
        println!("client received a packet:");
        let client_packet = read_packet(&mut client_read_stream);  
        send_packet(&mut server_send_stream, client_packet);
      }
    });
    println!("client listener spawned")
  }
}

fn read_packet(mut stream: &TcpStream) -> Vec<u8> {
  let mut packet_length_bits: Vec<u8> = Vec::new();
  loop {
    let buf: &mut [u8] = &mut [0];
    stream.read(buf).unwrap();
    packet_length_bits.push(buf[0]);

    if buf[0] & 0x80 == 0 {
      break;
    }
  }

  let packet_length = lib::deserialize::varint(&mut packet_length_bits).unwrap();
  println!("length: {packet_length}");

  let mut packet: Vec<u8> = vec![0; packet_length as usize];
  stream.read_exact(&mut packet).unwrap();

  println!("data: {packet:?}");

  return packet;
}

fn send_packet(mut stream: &TcpStream, mut packet: Vec<u8>) {
  let mut length_prefixed_packet: Vec<u8> = lib::serialize::varint(packet.len() as i32);
  length_prefixed_packet.append(&mut packet);

  stream.write(length_prefixed_packet.as_slice()).unwrap();
  stream.flush().unwrap();
}