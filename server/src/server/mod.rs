use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use lib::ConnectionState;

pub mod packet;

pub fn initialize_server() {
  let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
  
  let mut connection_states: HashMap<SocketAddr, ConnectionState> = HashMap::new();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    println!("New Connection from {}", stream.peer_addr().unwrap());

    loop {
      if handle_connection(&stream, &mut connection_states) {
        connection_states.remove(&stream.peer_addr().unwrap());
        break;
      }
    }
  }
}

fn handle_connection(stream: &TcpStream, connection_states: &mut HashMap<SocketAddr, ConnectionState>) -> bool {
  let mut stream = stream.try_clone().unwrap();
  let packet = lib::utils::read_packet(&mut stream);

  return packet::handlers::handle_packet(packet, &mut stream, connection_states);
}