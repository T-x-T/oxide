#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use lib::packets::Packet;
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
    path: Path::new(&std::env::var("OXIDE_WORLD_PATH").unwrap_or("./world".to_string())).to_owned()
  };

  let entity_id_manager = EntityIdManager::default();
  let mut game = Game {
    players: Mutex::new(Vec::new()),
    world: Mutex::new(World::new(world_loader, &entity_id_manager, &block_states)),
    entity_id_manager,
    commands: Mutex::new(Vec::new()),
    last_save_all_timestamp: Mutex::new(std::time::Instant::now()),
    last_player_keepalive_timestamp: Mutex::new(std::time::Instant::now()),
    block_state_data: block_states,
    connections: Mutex::new(HashMap::new()),
  };

  command::init(&mut game);

  let game: Arc<Game> = Arc::new(game);

  terminal_input::init(game.clone());

  let game_clone = game.clone();
  std::thread::spawn(move || main_loop(game_clone));

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    println!("New Connection from {}", stream.peer_addr().unwrap());
    let game_clone = game.clone();
    std::thread::spawn(move || {
      let mut stream = stream.try_clone().unwrap();
      let peer_addr = stream.peer_addr().unwrap();
      loop {
        let mut peek_buf = [0; 1];

        match stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            disconnect_player(&peer_addr, game_clone.clone());
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {e}");
            disconnect_player(&peer_addr, game_clone.clone());
            break;
          }
          _ => {}
        }

        let packet = lib::utils::read_packet(&stream);

        if stream.peer_addr().is_err() {
          disconnect_player(&peer_addr, game_clone.clone());
          break;
        }

        let packet_handler_result = packet_handlers::handle_packet(packet, &mut stream, game_clone.clone());
        if packet_handler_result.is_err() {
       		println!("got error, so lets disconnect: {}", packet_handler_result.err().unwrap());
         disconnect_player(&peer_addr, game_clone);
          break;
        }
        if packet_handler_result.is_ok_and(|x| x.is_some()) {
       		println!("handler told us to disconnect");
          disconnect_player(&peer_addr, game_clone);
          break;
        }
      }
    });
  }
}

fn disconnect_player(peer_addr: &SocketAddr, game: Arc<Game>) {
  let mut connections = game.connections.lock().unwrap();
  let mut players = game.players.lock().unwrap();
	let player_to_remove = players.iter().find(|x| x.peer_socket_address == *peer_addr);
	if let Some(player_to_remove) = player_to_remove {
    player_to_remove.save_to_disk();
    players.iter().for_each(|x| {
	    let _ = lib::utils::send_packet(&x.connection_stream, lib::packets::clientbound::play::PlayerInfoRemove::PACKET_ID, lib::packets::clientbound::play::PlayerInfoRemove {
	      uuids: vec![player_to_remove.uuid],
	    }.try_into().unwrap());

			let _ = lib::utils::send_packet(&x.connection_stream, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, lib::packets::clientbound::play::RemoveEntities {
        entity_ids: vec![player_to_remove.entity_id]
      }.try_into().unwrap());
    });
	}

  connections.remove(peer_addr);

  drop(connections);
  players.retain(|x| x.peer_socket_address != *peer_addr);
}

fn main_loop(game: Arc<Game>) {
  loop {
    let start_time = std::time::Instant::now();

    let tick_timings = tick(game.clone());

    let end_time = std::time::Instant::now();
    let tick_duration = end_time - start_time;

    if std::time::Duration::from_millis(50) > tick_duration {
      std::thread::sleep(std::time::Duration::from_millis(50) - tick_duration);
    } else {
      println!("tick took longer than 50ms! It finished in {tick_duration:.2?}\n{tick_timings:?}");
    }
  }
}

#[derive(Debug)]
pub struct TickTimings {
  pub save_all: std::time::Duration,
  pub clone_players: std::time::Duration,
  pub send_keepalives: std::time::Duration,
  pub tick_blockentities: std::time::Duration,
  pub tick_entities: std::time::Duration,
}

fn tick(game: Arc<Game>) -> TickTimings {
  let now = std::time::Instant::now();
  // TODO: Make a better way to handle configuration to avoid repeated and complex code
  // all this just to get the number to a u64
  // also shouldn't be run every tick but just once in initialize_server()
  let save_interval: u64 = std::env::var("OXIDE_SAVE_SECONDS")
  .unwrap_or("60".to_string())
  .parse::<u64>()
  .unwrap_or(60);

  if std::time::Instant::now() > *game.last_save_all_timestamp.lock().unwrap() + std::time::Duration::from_secs(save_interval) {
    println!("run save-all");
    game.save_all();
  }
  let duration_save_all = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  let players = game.players.lock().unwrap().clone();
  let duration_clone_players = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  if std::time::Instant::now() > *game.last_player_keepalive_timestamp.lock().unwrap() + std::time::Duration::from_secs(5) {
    *game.last_player_keepalive_timestamp.lock().unwrap() = std::time::Instant::now();

    let players = players.clone();
    let game = game.clone();
    std::thread::spawn(move || {
      for player in &players {
        let useless_buf_no_one_crates_about = &mut [0; 1];
        if player.connection_stream.peek(useless_buf_no_one_crates_about).is_err() {
          disconnect_player(&player.peer_socket_address, game.clone());
        }
        if lib::utils::send_packet(&player.connection_stream, lib::packets::clientbound::play::ClientboundKeepAlive::PACKET_ID, lib::packets::clientbound::play::ClientboundKeepAlive {
          keep_alive_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        }.try_into().unwrap()).is_err() {
          disconnect_player(&player.peer_socket_address, game.clone());
        };
      }
    });
  }
  let duration_send_keepalives = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  for dimension in &mut game.world.lock().unwrap().dimensions {
    for chunk in &mut dimension.1.chunks {
      for blockentity in &mut chunk.block_entities {
        if blockentity.needs_ticking {
          blockentity.tick(&players);
        }
      }
    }
  }
  let duration_tick_blockentities = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  for dimension in &mut game.world.lock().unwrap().dimensions {
    let mut entities = std::mem::take(&mut dimension.1.entities);
    let mut entity_tick_outcomes: Vec<(i32, EntityTickOutcome)> = Vec::new();
    for entity in &mut entities {
      let outcome = entity.tick(dimension.1, &players, &game.block_state_data);
      //println!("ticked entity in {:.2?}", std::time::Instant::now() - now);
      if outcome != EntityTickOutcome::None {
        entity_tick_outcomes.push((entity.get_common_entity_data().entity_id, outcome));
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

          if let Some(chunk) = dimension.1.get_chunk_from_position_mut(
            dimension.1.entities
              .iter()
              .find(|x| x.get_common_entity_data().entity_id == outcome.0)
              .unwrap()
              .get_common_entity_data()
              .position.into()
          ) {
            chunk.modified = true;
          };
          dimension.1.entities.retain(|x| x.get_common_entity_data().entity_id != outcome.0);
        },
        EntityTickOutcome::Updated => {
          if let Some(chunk) = dimension.1.get_chunk_from_position_mut(
            dimension.1.entities
              .iter()
              .find(|x| x.get_common_entity_data().entity_id == outcome.0)
              .unwrap()
              .get_common_entity_data()
              .position.into()
          ) {
            chunk.modified = true;
          };
        },
        EntityTickOutcome::None => (),
      }
    }
  }
  let duration_tick_entities = std::time::Instant::now() - now;

  return TickTimings {
    save_all: duration_save_all,
    clone_players: duration_clone_players,
    send_keepalives: duration_send_keepalives,
    tick_blockentities: duration_tick_blockentities,
    tick_entities: duration_tick_entities,
  }
}
