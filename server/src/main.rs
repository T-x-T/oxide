#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use lib::packets::Packet;
use lib::ConnectionState;
use lib::types::*;

mod packet_handlers;
mod command;
mod terminal_input;

fn main() {
  println!("Starting the oxide server");
  initialize_server();
}

fn initialize_server() {
  let listener = TcpListener::bind(std::env::var("OXIDE_LISTEN_ON").unwrap_or("0.0.0.0:25565".to_string())).unwrap();
  println!("oxide listening on {}", listener.local_addr().unwrap());

  let block_states = data::blocks::get_blocks();

  let world_loader = lib::world::loader::vanilla::Loader {
    path: Path::new(&std::env::var("OXIDE_WORLD_PATH").unwrap_or("./world".to_string())).to_owned(),
    block_states,
  };

  let connections: Arc<Mutex<HashMap<SocketAddr, Connection>>> = Arc::new(Mutex::new(HashMap::new()));
  let next_entity_id: &mut i32 = &mut 0;
  let mut game = Game {
    players: Vec::new(),
    world: World::new(world_loader, next_entity_id),
    last_created_entity_id: 0,
    chat_message_index: 0,
    commands: Vec::new(),
    last_save_all_timestamp: std::time::Instant::now(),
  };
  game.last_created_entity_id = *next_entity_id;
  command::init(&mut game);

  let game: Arc<Mutex<Game>> = Arc::new(Mutex::new(game));

  let connection_streams: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

  terminal_input::init(connection_streams.clone(), game.clone(), connections.clone());

  let game_clone = game.clone();
  std::thread::spawn(move || main_loop(game_clone));

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
    player_to_remove.save_to_disk();
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

fn main_loop(game: Arc<Mutex<Game>>) {
  loop {
    let start_time = std::time::Instant::now();

    tick(game.clone());

    let end_time = std::time::Instant::now();
    let tick_duration = end_time - start_time;

    if std::time::Duration::from_millis(50) > tick_duration {
      std::thread::sleep(std::time::Duration::from_millis(50) - tick_duration);
    } else {
      println!("tick took longer than 50ms! It finished in {tick_duration:.2?}");
    }
  }
}

fn tick(game: Arc<Mutex<Game>>) {
  // TODO: Make a better way to handle configuration to avoid repeated and complex code
  // all this just to get the number to a u64
  // also shouldn't be run every tick but just once in initialize_server()
  let save_interval: u64 = std::env::var("OXIDE_SAVE_SECONDS")
  .unwrap_or("60".to_string())
  .parse::<u64>()
  .unwrap_or(60);

  if std::time::Instant::now() > game.lock().unwrap().last_save_all_timestamp + std::time::Duration::from_secs(save_interval) {
    println!("run save-all");
    game.lock().unwrap().save_all();
  }

  let mut game = game.lock().unwrap();
  let players = game.players.clone();
  for dimension in &mut game.world.dimensions {
    for chunk in &mut dimension.1.chunks {
      for blockentity in &mut chunk.block_entities {
        if blockentity.needs_ticking {
          blockentity.tick(&players);
        }
      }
    }

    let mut entities = std::mem::take(&mut dimension.1.entities);
    let mut entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = Vec::new();
    for entity in &mut entities {
      let chunk = dimension.1.get_chunk_from_position(entity.get_common_entity_data().position.into());
      if let Some(chunk) = chunk {
        let outcome = entity.tick(chunk, &players);
        if outcome != EntityTickOutcome::None {
          entity_tick_outcomes.push((entity.get_common_entity_data().entity_id, outcome));
        }
      }
    }
    dimension.1.entities = entities;
    for outcome in entity_tick_outcomes {
      match outcome.1 {
        EntityTickOutcome::SelfDied => {
          let entity_event_packet = lib::packets::clientbound::play::EntityEvent {
            entity_id: outcome.0,
            entity_status: 3,
          };

          for player in &players {
            lib::utils::send_packet(&player.connection_stream, lib::packets::clientbound::play::EntityEvent::PACKET_ID, entity_event_packet.clone().try_into().unwrap()).unwrap();
          }
        },
        EntityTickOutcome::RemoveSelf => {
          let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
            entity_ids: vec![outcome.0],
          };

          for player in &players {
            lib::utils::send_packet(&player.connection_stream, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, remove_entities_packet.clone().try_into().unwrap()).unwrap();
          }

          dimension.1.entities.retain(|x| x.get_common_entity_data().entity_id != outcome.0);
        }
        EntityTickOutcome::None => (),
      }
    }
  }
  drop(game);
}
