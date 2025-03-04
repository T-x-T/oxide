use std::net::{TcpListener, TcpStream};
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

    let connection = Arc::new(Mutex::new(Connection { state: lib::ConnectionStates::Handshaking, protocol_version: 0 }));
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

        let server_packet = lib::utils::read_packet(&mut server_read_stream);
        
        let packet_id = server_packet.id;
        println!("received serverbound packet: {packet_id} {:#04x}", packet_id);
        //println!("data: {server_packet:?}");
        
        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          lib::ConnectionStates::Handshaking => {
            if packet_id == 0x00 {
              let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(server_packet.data.clone()).unwrap();
              println!("parsed Handshake packet: {parsed_packet:?}");
              let new_connection_data = Connection {state: parsed_packet.next_state.into(), protocol_version: parsed_packet.protocol_version};
              *connection.lock().unwrap() = new_connection_data.clone();
              println!("Set state to {:?} and version to {}", new_connection_data.state, new_connection_data.protocol_version);
            }
          },
          lib::ConnectionStates::Status => {
          },
          lib::ConnectionStates::Login => {
            if packet_id == 0x00 {
              let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(server_packet.data.clone()).unwrap();
              println!("parsed LoginStart packet: {parsed_packet:?}");
            }
            if packet_id == 0x03 {
              let parsed_packet = lib::packets::serverbound::login::LoginAcknowledged::try_from(server_packet.data.clone()).unwrap();
              println!("parsed LoginAcknowledged packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionStates::Configuration;
              println!("changed state to configuration");
            }
          },
          lib::ConnectionStates::Configuration => {
            if packet_id == 0x07 {
              let parsed_packet = lib::packets::serverbound::configuration::ServerboundKnownPackets::try_from(server_packet.data.clone()).unwrap();
              println!("parsed ServerBoundKnownPackets packet: {parsed_packet:?}");
            }
          },
          lib::ConnectionStates::Play => {
            
          },
          lib::ConnectionStates::Transfer => {
            
          },
        }

        lib::utils::send_packet(&mut client_send_stream, packet_id as u8, server_packet.data);
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
  
        let client_packet = lib::utils::read_packet(&mut client_read_stream);
        
        let packet_id = client_packet.id;
        println!("received clientbound packet: {packet_id} {:#04x}", packet_id);
        //println!("data: {client_packet:?}");

        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          lib::ConnectionStates::Handshaking => {
            
          },
          lib::ConnectionStates::Status => {
            
          },
          lib::ConnectionStates::Login => {
            if packet_id == 0x02 {
              let parsed_packet = lib::packets::clientbound::login::LoginSuccess::try_from(client_packet.data.clone()).unwrap();
              println!("parsed LoginSuccess packet: {parsed_packet:?}");
            }
          },
          lib::ConnectionStates::Configuration => {
            if packet_id == 0x0e {
              let parsed_packet = lib::packets::clientbound::configuration::ClientboundKnownPacks::try_from(client_packet.data.clone()).unwrap();
              println!("parsed ClientBoundKnownPackets packet: {parsed_packet:?}");
            }
            if packet_id == 0x07 {
              let parsed_packet = lib::packets::clientbound::configuration::RegistryData::try_from(client_packet.data.clone()).unwrap();
              println!("parsed RegistryData packet: {parsed_packet:?}");
            }
          },
          lib::ConnectionStates::Play => {
            
          },
          lib::ConnectionStates::Transfer => {
            
          },
        }

        lib::utils::send_packet(&mut server_send_stream, packet_id as u8, client_packet.data);
      }
    });
    println!("client listener spawned")
  }
}

#[derive(Debug, Clone)]
struct Connection {
  state: lib::ConnectionStates,
  protocol_version: i32,
}