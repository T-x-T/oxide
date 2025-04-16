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

    let connection = Arc::new(Mutex::new(Connection { state: lib::ConnectionState::Handshaking, protocol_version: 0 }));
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
        let mut parsed_server_packet: Option<Vec<u8>> = None;
        
        let packet_id = server_packet.id;
        println!("received serverbound packet: {packet_id} {:#04x}", packet_id);
        //println!("data: {:?}", server_packet.raw_data);
        
        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          lib::ConnectionState::Handshaking => {
            if packet_id == 0x00 {
              let parsed_packet = lib::packets::serverbound::handshaking::Handshake::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              let new_connection_data = Connection {state: parsed_packet.next_state.clone().into(), protocol_version: parsed_packet.protocol_version};
              *connection.lock().unwrap() = new_connection_data.clone();
              println!("Set state to {:?} and version to {}", new_connection_data.state, new_connection_data.protocol_version);
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Status => {
          },
          lib::ConnectionState::Login => {
            if packet_id == 0x00 {
              let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(server_packet.data.clone()).unwrap();
              println!("parsed LoginStart packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x03 {
              let parsed_packet = lib::packets::serverbound::login::LoginAcknowledged::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Configuration;
              println!("changed state to configuration");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
            if packet_id == 0x07 {
              let parsed_packet = lib::packets::serverbound::configuration::ServerboundKnownPackets::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x03 {
              let parsed_packet = lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Play;
              println!("changed status to play");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Play => {
            if packet_id == 0x1c {
              let parsed_packet = lib::packets::serverbound::play::SetPlayerPosition::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Transfer => {
            
          },
        }

        if parsed_server_packet.is_some() {
          lib::utils::send_packet(&mut client_send_stream, packet_id as u8, parsed_server_packet.unwrap());
        } else {
          lib::utils::send_packet(&mut client_send_stream, packet_id as u8, server_packet.data);
        }
      }
    });
    println!("server listener spawned");
    
    //Handle packets coming in on the client side
    std::thread::spawn(move || {
      let connection = connection.clone();
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
        let mut parsed_client_packet: Option<Vec<u8>> = None;
        
        let packet_id = client_packet.id;
        println!("received clientbound packet: {packet_id} {:#04x}", packet_id);
        //println!("data: {:?}", client_packet.raw_data);

        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          lib::ConnectionState::Handshaking => {
            
          },
          lib::ConnectionState::Status => {
            if packet_id == 0x00 {
              let parsed_packet = lib::packets::clientbound::status::StatusResponse::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Login => {
            if packet_id == 0x02 {
              let parsed_packet = lib::packets::clientbound::login::LoginSuccess::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
            if packet_id == 0x0e {
              let parsed_packet = lib::packets::clientbound::configuration::ClientboundKnownPacks::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x07 {
              let parsed_packet = lib::packets::clientbound::configuration::RegistryData::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x03 {
              let parsed_packet = lib::packets::clientbound::configuration::FinishConfiguration::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Play => {
            if packet_id == 0x01 {
              let parsed_packet = lib::packets::clientbound::play::SpawnEntity::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x2f {
              let parsed_packet = lib::packets::clientbound::play::UpdateEntityPosition::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x30 {
              let parsed_packet = lib::packets::clientbound::play::UpdateEntityPositionAndRotation::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x2c {
              let parsed_packet = lib::packets::clientbound::play::Login::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x23 {
              let parsed_packet = lib::packets::clientbound::play::GameEvent::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            /* if packet_id == 0x28 {
              //println!("before parsing:\n\n{:?}\n\n", client_packet.data);
              let parsed_packet = lib::packets::clientbound::play::ChunkDataAndUpdateLight::try_from(client_packet.data.clone());
              if parsed_packet.is_ok() {
                //println!("parsed packet: {parsed_packet:?}");

                let serialized: Result<Vec<u8>, _> = lib::packets::clientbound::play::ChunkDataAndUpdateLight::try_into(parsed_packet.as_ref().unwrap().clone());
                if serialized.is_ok() {
                  let parsed_second_time: Result<lib::packets::clientbound::play::ChunkDataAndUpdateLight, _> = serialized.unwrap().try_into();
                  if parsed_second_time.is_err() {
                    println!("got error trying to parse second time:{}", parsed_second_time.as_ref().clone().err().unwrap());
                    println!("before parsing:\n\n{:?}\n\n", client_packet.data);
                    println!("parsed:\n\n{:?}\n\n", parsed_packet);
                    println!("after parsing:\n\n{:?}\n\n", parsed_second_time.unwrap());
                  }
                } else {
                  println!("got error trying to serialize again:{}", serialized.err().unwrap());
                }


                //println!("parsed:\n\n{:?}\n\n", parsed_packet);
                parsed_client_packet = Some(parsed_packet.unwrap().try_into().unwrap());
                //println!("after parsing:\n\n{:?}\n\n", parsed_client_packet.clone().unwrap());
              } else {
                println!("got error trying to parse ChunkDataAndUpdateLight packet: {}", parsed_packet.err().unwrap());
                println!("before parsing:\n\n{:?}\n\n", client_packet.data);
              }
            } */
            if packet_id == 0x40 {
              let parsed_packet = lib::packets::clientbound::play::PlayerInfoUpdate::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              //no idea why this crashes the client
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == 0x5d {
              let parsed_packet = lib::packets::clientbound::play::SetEntityMetadata::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Transfer => {
            
          },
        }

        if parsed_client_packet.is_some() {
          lib::utils::send_packet(&mut server_send_stream, packet_id as u8, parsed_client_packet.unwrap());
        } else {
          lib::utils::send_packet(&mut server_send_stream, packet_id as u8, client_packet.data);
        }
      }
    });
    println!("client listener spawned")
  }
}

#[derive(Debug, Clone)]
struct Connection {
  state: lib::ConnectionState,
  protocol_version: i32,
}