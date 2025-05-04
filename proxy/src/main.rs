use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use lib::packets::Packet;

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
            if packet_id == lib::packets::serverbound::login::LoginStart::get_id() {
              let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(server_packet.data.clone()).unwrap();
              println!("parsed LoginStart packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::serverbound::login::LoginAcknowledged::get_id() {
              let parsed_packet = lib::packets::serverbound::login::LoginAcknowledged::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Configuration;
              println!("changed state to configuration");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
            if packet_id == lib::packets::serverbound::configuration::ServerboundKnownPackets::get_id() {
              let parsed_packet = lib::packets::serverbound::configuration::ServerboundKnownPackets::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::get_id() {
              let parsed_packet = lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Play;
              println!("changed status to play");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Play => {
            if packet_id == lib::packets::serverbound::play::SetPlayerPosition::get_id() {
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
            if packet_id == lib::packets::clientbound::status::StatusResponse::get_id() {
              let parsed_packet = lib::packets::clientbound::status::StatusResponse::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Login => {
            if packet_id == lib::packets::clientbound::login::LoginSuccess::get_id() {
              let parsed_packet = lib::packets::clientbound::login::LoginSuccess::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
            if packet_id == lib::packets::clientbound::configuration::ClientboundKnownPacks::get_id() {
              let parsed_packet = lib::packets::clientbound::configuration::ClientboundKnownPacks::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::configuration::RegistryData::get_id() {
              let parsed_packet = lib::packets::clientbound::configuration::RegistryData::try_from(client_packet.data.clone()).unwrap();
              //println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::configuration::FinishConfiguration::get_id() {
              let parsed_packet = lib::packets::clientbound::configuration::FinishConfiguration::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::configuration::UpdateTags::get_id() {
              let parsed_packet = lib::packets::clientbound::configuration::UpdateTags::try_from(client_packet.data.clone()).unwrap();
              //println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Play => {
            if packet_id == lib::packets::clientbound::play::SpawnEntity::get_id() {
              let parsed_packet = lib::packets::clientbound::play::SpawnEntity::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::UpdateEntityPosition::get_id() {
              let parsed_packet = lib::packets::clientbound::play::UpdateEntityPosition::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::UpdateEntityPositionAndRotation::get_id() {
              let parsed_packet = lib::packets::clientbound::play::UpdateEntityPositionAndRotation::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::Login::get_id() {
              let parsed_packet = lib::packets::clientbound::play::Login::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::GameEvent::get_id() {
              let parsed_packet = lib::packets::clientbound::play::GameEvent::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::ChunkDataAndUpdateLight::get_id() {
              let parsed_packet = lib::packets::clientbound::play::ChunkDataAndUpdateLight::try_from(client_packet.data.clone()).unwrap();
              //println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::clientbound::play::PlayerInfoUpdate::get_id() {
              let parsed_packet = lib::packets::clientbound::play::PlayerInfoUpdate::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              //no idea why this crashes the client
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
            // Disabled because implementation is still incomplete
            // if packet_id == lib::packets::clientbound::play::SetEntityMetadata::get_id() {
            //   let parsed_packet = lib::packets::clientbound::play::SetEntityMetadata::try_from(client_packet.data.clone()).unwrap();
            //   println!("parsed packet: {parsed_packet:?}");
            //   parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            // }
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
