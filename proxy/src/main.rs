use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};

fn main() {
  println!("Starting the oxide proxy");
  
  let listener = TcpListener::bind("127.0.0.1:35565").unwrap();
  
  for stream in listener.incoming() {
    let mut server_read_stream = stream.unwrap();
    println!("New Connection from {}", server_read_stream.peer_addr().unwrap());

    let mut client_send_stream = TcpStream::connect("127.0.0.1:25565").unwrap();
    let mut client_read_stream = client_send_stream.try_clone().unwrap();
    let mut server_send_stream = server_read_stream.try_clone().unwrap();

    let connection = Arc::new(Mutex::new(Connection { state: ConnectionStates::Handshaking, protocol_version: 0 }));
    let connection_clone = connection.clone();

    //Handle packets coming in on the server side
    std::thread::spawn(move || {
      let connection = connection_clone;
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
        
        let packet_id = lib::deserialize::varint(&mut server_packet.clone()).unwrap();
        println!("packet_id: {packet_id} {:#04x}", packet_id);
        println!("data: {server_packet:?}");
        
        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          ConnectionStates::Handshaking => {
            println!("we are in the handshaking state");
            if packet_id == 0x00 {
              let parsed_packet = packets::serverbound::handshaking::Handshake::try_from(server_packet.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              let new_connection_data = Connection {state: parsed_packet.next_state.into(), protocol_version: parsed_packet.protocol_version};
              *connection.lock().unwrap() = new_connection_data.clone();
              println!("Set state to {:?} and version to {}", new_connection_data.state, new_connection_data.protocol_version);
            }
          },
          ConnectionStates::Status => {
            println!("we are in the status state");
          },
          ConnectionStates::Login => {
            println!("we are in the login state");
          },
          ConnectionStates::Configuration => {
            println!("we are in the configuration state");
          },
          ConnectionStates::Play => {
            println!("we are in the play state");
          },
          ConnectionStates::Transfer => {
            println!("we are in the transfer state");
          },
        }

        send_packet(&mut client_send_stream, server_packet);
      }
    });
    println!("server listeneder spawned");
    
    //Handle packets coming in on the client side
    std::thread::spawn(move || {
      let conenction = connection.clone();
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
        
        let packet_id = lib::deserialize::varint(&mut client_packet.clone()).unwrap();
        println!("packet_id: {packet_id} {:#04x}", packet_id);
        println!("data: {client_packet:?}");

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
  let mut packet: Vec<u8> = vec![0; packet_length as usize];
  stream.read_exact(&mut packet).unwrap();

  return packet;
}

fn send_packet(mut stream: &TcpStream, mut packet: Vec<u8>) {
  let mut length_prefixed_packet: Vec<u8> = lib::serialize::varint(packet.len() as i32);
  length_prefixed_packet.append(&mut packet);

  stream.write(length_prefixed_packet.as_slice()).unwrap();
  stream.flush().unwrap();
}

#[derive(Debug, Clone)]
struct Connection {
  state: ConnectionStates,
  protocol_version: i32,
}

#[derive(Debug, Clone)]
pub enum ConnectionStates {
  Handshaking,
  Transfer,
  Status,
  Login,
  Configuration,
  Play,
}

pub mod packets {
  use std::error::Error;
  pub mod serverbound {
    use super::*;
    pub mod handshaking {
      use super::*;
      #[derive(Debug, Clone)]
      pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub sever_port: u16,
        pub next_state: HandshakeNextStates,
      }

      #[derive(Debug, Clone)]
      pub enum HandshakeNextStates {
        Status = 1,
        Login = 2,
        Transfer = 3,
      }

      impl TryFrom<i32> for HandshakeNextStates {
        type Error = Box<dyn Error>;
        
        fn try_from(value: i32) -> Result<Self, Self::Error> {
          match value {
            1 => Ok(HandshakeNextStates::Status),
            2 => Ok(HandshakeNextStates::Login),
            3 => Ok(HandshakeNextStates::Transfer),
            _ => return Err(Box::new(lib::CustomError::ParseInvalidValue)),
          }
        }
      }

      impl From<HandshakeNextStates> for crate::ConnectionStates {
        fn from(value: HandshakeNextStates) -> Self {
          match value {
            HandshakeNextStates::Status => return Self::Status,
            HandshakeNextStates::Login => return Self::Login,
            HandshakeNextStates::Transfer => return Self::Transfer,
          }
        }
      }

      impl TryFrom<Vec<u8>> for Handshake {
        type Error = Box<dyn Error>;

        fn try_from(mut value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
          value.remove(0);
          return Ok(Handshake {
            protocol_version: lib::deserialize::varint(&mut value)?,
            server_address: lib::deserialize::string(&mut value)?,
            sever_port: lib::deserialize::unsigned_short(&mut value)?,
            next_state: lib::deserialize::varint(&mut value)?.try_into()?,
          })
        }
      }
    }
  }
}