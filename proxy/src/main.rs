#![allow(clippy::needless_return)]

use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use lib::packets::Packet;

fn main() {
  println!("Starting the oxide proxy");

  let listener = TcpListener::bind("127.0.0.1:35565").unwrap();

  for stream in listener.incoming() {
    let server_read_stream = stream.unwrap();
    println!("New Connection from {}", server_read_stream.peer_addr().unwrap());

    let client_send_stream = TcpStream::connect("127.0.0.1:25565").unwrap();
    let client_read_stream = client_send_stream.try_clone().unwrap();
    let server_send_stream = server_read_stream.try_clone().unwrap();

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
            eprintln!("error reading from server: {e}");
            break;
          }
          _ => {}
        }

        let server_packet = lib::utils::read_packet(&server_read_stream);
        let mut parsed_server_packet: Option<Vec<u8>> = None;

        let packet_id = server_packet.id;
        //println!("received serverbound packet: {packet_id} {:#04x}", packet_id);
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
            if packet_id == lib::packets::serverbound::login::LoginStart::PACKET_ID {
              let parsed_packet = lib::packets::serverbound::login::LoginStart::try_from(server_packet.data.clone()).unwrap();
              println!("parsed LoginStart packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::serverbound::login::LoginAcknowledged::PACKET_ID {
              let parsed_packet = lib::packets::serverbound::login::LoginAcknowledged::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Configuration;
              println!("changed state to configuration");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
            if packet_id == lib::packets::serverbound::configuration::ServerboundKnownPackets::PACKET_ID {
              let parsed_packet = lib::packets::serverbound::configuration::ServerboundKnownPackets::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
            if packet_id == lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::PACKET_ID {
              let parsed_packet = lib::packets::serverbound::configuration::AcknowledgeFinishConfiguration::try_from(server_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              connection.lock().unwrap().state = lib::ConnectionState::Play;
              println!("changed status to play");
              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Play => {
          	match packet_id {
           		lib::packets::serverbound::play::SetPlayerPosition::PACKET_ID => {
								// let parsed_packet = lib::packets::serverbound::play::SetPlayerPosition::try_from(server_packet.data.clone()).unwrap();
								// println!("parsed packet: {parsed_packet:?}");
								// parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::SetPlayerPositionAndRotation::PACKET_ID => {
	              // let parsed_packet = lib::packets::serverbound::play::SetPlayerPositionAndRotation::try_from(server_packet.data.clone()).unwrap();
	              // println!("parsed packet: {parsed_packet:?}");
	              // parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::SetPlayerRotation::PACKET_ID => {
	              // let parsed_packet = lib::packets::serverbound::play::SetPlayerRotation::try_from(server_packet.data.clone()).unwrap();
	              // println!("parsed packet: {parsed_packet:?}");
	              // parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::PlayerAction::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::PlayerAction::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::SetHandItem::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::SetHandItem::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::SetCreativeModeSlot::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::SetCreativeModeSlot::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::UseItemOn::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::UseItemOn::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::serverbound::play::ChatMessage::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::ChatMessage::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              }
              lib::packets::serverbound::play::Interact::PACKET_ID => {
	              let parsed_packet = lib::packets::serverbound::play::Interact::try_from(server_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_server_packet = Some(parsed_packet.try_into().unwrap());
              }
            	_ => (),
            };
          },
          lib::ConnectionState::Transfer => {

          },
        }

        match parsed_server_packet {
          Some(x) => lib::utils::send_packet(&client_send_stream, packet_id, x).unwrap(),
          None => lib::utils::send_packet(&client_send_stream, packet_id, server_packet.data).unwrap(),
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
            eprintln!("error reading from client: {e}");
            break;
          }
          _ => {}
        }

        let client_packet = lib::utils::read_packet(&client_read_stream);
        let mut parsed_client_packet: Option<Vec<u8>> = None;

        let packet_id = client_packet.id;
        //println!("received clientbound packet: {packet_id} {:#04x}", packet_id);
        //println!("data: {:?}", client_packet.raw_data);

        let current_state = connection.lock().unwrap().state.clone();

        match current_state {
          lib::ConnectionState::Handshaking => {

          },
          lib::ConnectionState::Status => {
            if packet_id == lib::packets::clientbound::status::StatusResponse::PACKET_ID {
              let parsed_packet = lib::packets::clientbound::status::StatusResponse::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Login => {
            if packet_id == lib::packets::clientbound::login::LoginSuccess::PACKET_ID {
              let parsed_packet = lib::packets::clientbound::login::LoginSuccess::try_from(client_packet.data.clone()).unwrap();
              println!("parsed packet: {parsed_packet:?}");
              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
            }
          },
          lib::ConnectionState::Configuration => {
          	match packet_id {
           		lib::packets::clientbound::configuration::ClientboundKnownPacks::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::ClientboundKnownPacks::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::clientbound::configuration::RegistryData::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::RegistryData::try_from(client_packet.data.clone()).unwrap();
	              //println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::clientbound::configuration::FinishConfiguration::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::FinishConfiguration::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::clientbound::configuration::ShowDialog::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::ShowDialog::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::clientbound::configuration::ServerLinks::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::ServerLinks::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              },
              lib::packets::clientbound::configuration::UpdateTags::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::configuration::UpdateTags::try_from(client_packet.data.clone()).unwrap();
	              //println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
              }
           		_ => {
           			println!("received unkown packet in clientbound::configuration with id 0x{packet_id:02x}");
              },
           	};
          },
          lib::ConnectionState::Play => {
          	match packet_id {
           		lib::packets::clientbound::play::SpawnEntity::PACKET_ID => {
	            	//	let parsed_packet = lib::packets::clientbound::play::SpawnEntity::try_from(client_packet.data.clone()).unwrap();
		            // println!("parsed packet: {parsed_packet:?}");
		            // parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::UpdateEntityPosition::PACKET_ID => {
								// let parsed_packet = lib::packets::clientbound::play::UpdateEntityPosition::try_from(client_packet.data.clone()).unwrap();
								// println!("parsed packet: {parsed_packet:?}");
								// parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::UpdateEntityPositionAndRotation::PACKET_ID => {
								// let parsed_packet = lib::packets::clientbound::play::UpdateEntityPositionAndRotation::try_from(client_packet.data.clone()).unwrap();
								// println!("parsed packet: {parsed_packet:?}");
								// parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID => {
								let parsed_packet = lib::packets::clientbound::play::UpdateEntityRotation::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::Login::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::Login::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::GameEvent::PACKET_ID => {
								let parsed_packet = lib::packets::clientbound::play::GameEvent::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::ChunkDataAndUpdateLight::PACKET_ID => {
		        		let parsed_packet = lib::packets::clientbound::play::ChunkDataAndUpdateLight::try_from(client_packet.data.clone()).unwrap();
		            println!("parsed packet: {parsed_packet:?}");
		            parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::PlayerInfoUpdate::PACKET_ID => {
								let parsed_packet = lib::packets::clientbound::play::PlayerInfoUpdate::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID => {
	              let parsed_packet = lib::packets::clientbound::play::AcknowledgeBlockChange::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::BlockUpdate::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::BlockUpdate::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::PlayerChatMessage::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::PlayerChatMessage::try_from(client_packet.data.clone()).unwrap();
	              println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::Commands::PACKET_ID => {
	        			//let parsed_packet = lib::packets::clientbound::play::Commands::try_from(client_packet.data.clone()).unwrap();
								//println!("parsed packet: {parsed_packet:?}");
	              //parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::SetPlayerInventorySlot::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::SetPlayerInventorySlot::try_from(client_packet.data.clone()).unwrap();
								println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::SetHeldItem::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::SetHeldItem::try_from(client_packet.data.clone()).unwrap();
								println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::SetContainerContent::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::SetContainerContent::try_from(client_packet.data.clone()).unwrap();
								println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::ServerLinks::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::ServerLinks::try_from(client_packet.data.clone()).unwrap();
								println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::BlockEntityData::PACKET_ID => {
	        			let parsed_packet = lib::packets::clientbound::play::BlockEntityData::try_from(client_packet.data.clone()).unwrap();
								println!("parsed packet: {parsed_packet:?}");
	              parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
							lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID => {
		        		// Disabled because implementation is still incomplete
		            // let parsed_packet = lib::packets::clientbound::play::SetEntityMetadata::try_from(client_packet.data.clone()).unwrap();
		            // println!("parsed packet: {parsed_packet:?}");
		            // parsed_client_packet = Some(parsed_packet.try_into().unwrap());
							},
           		_ => {
           			//println!("unkown clientbound packet received with id: 0x{packet_id:02x}");
              },
           	};
          },
          lib::ConnectionState::Transfer => {

          },
        }

        match parsed_client_packet {
          Some(x) => lib::utils::send_packet(&server_send_stream, packet_id, x).unwrap(),
          None => lib::utils::send_packet(&server_send_stream, packet_id, client_packet.data).unwrap(),
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
