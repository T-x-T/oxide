#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use lib::packets::Packet;
use lib::ConnectionState;
use types::*;

mod packet_handlers;
mod types;
mod command;
mod terminal_input;

fn main() {
  println!("Starting the oxide server");
  initialize_server();
}

fn initialize_server() {
  let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

  let block_states = data::blocks::get_blocks();

  let world_loader = lib::world::loader::vanilla::Loader {
    path: Path::new("./world").to_owned(),
    block_states,
  };

  let connections: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
  let mut game = Game {
    players: Vec::new(),
    world: World::new(world_loader),
    last_created_entity_id: 0,
    chat_message_index: 0,
    commands: Vec::new(),
  };
  command::init(&mut game);

  let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(game));

  let connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  terminal_input::init(connection_streams.clone(), game.clone(), connections.clone());

  for stream in listener.incoming() {
    let stream = stream.unwrap();


    println!("New Connection from {}", stream.peer_addr().unwrap());
    let connections_clone = connections.clone();
    let connection_streams_clone = connection_streams.clone();
    let game_clone = game.clone();
    std::thread::spawn(move || {
      let mut stream = stream.try_clone().unwrap();
      let peer_addr = stream.peer_addr().unwrap();
      loop {
        let mut peek_buf = [0; 1];

        match stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {e}");
            disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
            break;
          }
          _ => {}
        }

        let packet = lib::utils::read_packet(&stream);

        if stream.peer_addr().is_err() {
          disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
          break;
        }

        let packet_handler_result = packet_handlers::handle_packet(packet, &mut stream, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap());
        if packet_handler_result.is_err() {
       		println!("got error, so lets disconnect: {}", packet_handler_result.err().unwrap());
          disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
          break;
        }
        if packet_handler_result.is_ok_and(|x| x.is_some()) {
       		println!("handler told us to disconnect");
          disconnect_player(&peer_addr, &mut connections_clone.lock().unwrap(), &mut connection_streams_clone.lock().unwrap(), &mut game_clone.lock().unwrap().players);
          break;
        }
      }
    });
  }
}

fn disconnect_player(peer_addr: &SocketAddr, connections: &mut HashMap<SocketAddr, Connection>, connection_streams: &mut HashMap<SocketAddr, TcpStream>, players: &mut Vec<Player>) {
	let player_to_remove = players.iter().find(|x| x.peer_socket_address == *peer_addr);
	if let Some(player_to_remove) = player_to_remove {
		connection_streams.iter()
	    .filter(|x| connections.get(x.0).is_some_and(|x| x.state == ConnectionState::Play))
	    .for_each(|x| {
		    let _ = lib::utils::send_packet(x.1, lib::packets::clientbound::play::PlayerInfoRemove::PACKET_ID, lib::packets::clientbound::play::PlayerInfoRemove {
		      uuids: vec![player_to_remove.uuid],
		    }.try_into().unwrap());

				let _ = lib::utils::send_packet(x.1, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, lib::packets::clientbound::play::RemoveEntities {
          entity_ids: vec![player_to_remove.entity_id]
        }.try_into().unwrap());
	    });
	}

  connections.remove(peer_addr);
  connection_streams.remove(peer_addr);
  players.retain(|x| x.peer_socket_address != *peer_addr);
}
