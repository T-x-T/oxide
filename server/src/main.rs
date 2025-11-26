#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use lib::packets::Packet;
use lib::types::*;
use lib::game::PacketHandlerAction;

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
    packet_handler_actions: Mutex::new(Vec::new()),
    packet_send_queues: DashMap::new(),
  };

  command::init(&mut game);

  let game: Arc<Game> = Arc::new(game);

  terminal_input::init(game.clone());

  let game_clone = game.clone();
  std::thread::spawn(move || main_loop(game_clone));

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    //RX
    println!("New Connection from {}", stream.peer_addr().unwrap());
    let game_clone = game.clone();
    let stream_clone = stream.try_clone().unwrap();
    std::thread::spawn(move || {
      let mut stream = stream_clone;
      let game = game_clone.clone();
      let peer_addr = stream.peer_addr().unwrap();
      loop {
        let mut peek_buf = [0; 1];

        match stream.peek(&mut peek_buf) {
          Ok(0) => {
            println!("client disconnected.");
            disconnect_player(&peer_addr, game.clone());
            break;
          }
          Err(e) => {
            eprintln!("error reading from client: {e}");
            disconnect_player(&peer_addr, game.clone());
            break;
          }
          _ => {}
        }

        let packet = lib::utils::read_packet(&stream);

        // if stream.peer_addr().is_err() {
        //   disconnect_player(&peer_addr, game.clone());
        //   break;
        // }

        let packet_handler_result = packet_handlers::handle_packet(packet, &mut stream, game.clone());
        if packet_handler_result.is_err() {
       		println!("got error, so lets disconnect: {}", packet_handler_result.err().unwrap());
          disconnect_player(&peer_addr, game);
          break;
        }
        if let Ok(packet_handler_result) = packet_handler_result && let Some(packet_handler_result) = packet_handler_result {
          game.packet_handler_actions.lock().unwrap().push(packet_handler_result);
        }
      }
    });


    //TX
    let game_clone = game.clone();
    std::thread::spawn(move || {
      let stream = stream.try_clone().unwrap();
      let game = game_clone.clone();
      let Ok(peer_addr) = stream.peer_addr() else { return; };

      loop {
        let Some(mut queue) = game.packet_send_queues.get_mut(&peer_addr) else { continue; };
        if !queue.is_empty() {
          let packet = queue.remove(0);
          drop(queue);
          let _ = send_packet(&stream, packet.0, packet.1).inspect_err(|x| println!("got error \"{x}\" trying to send packet id \"{}\" to stream \"{}\"", packet.0, stream.peer_addr().unwrap()));
        } else {
          drop(queue);
        };
      }
    });
  }
}

//copy of this function exists in proxy too
fn send_packet(mut stream: &TcpStream, packet_id: u8, mut data: Vec<u8>) -> Result<(), Box<dyn Error>> {
  let mut serialized_id: Vec<u8> = lib::serialize::varint(packet_id as i32);
  let mut packet: Vec<u8> = lib::serialize::varint((data.len() + serialized_id.len()) as i32);
  packet.append(&mut serialized_id);
  packet.append(&mut data);

  stream.write_all(packet.as_slice())?;
  stream.flush()?;

  return Ok(());
}

fn disconnect_player(peer_addr: &SocketAddr, game: Arc<Game>) {
  let mut connections = game.connections.lock().unwrap();
  let mut players = game.players.lock().unwrap();
	let player_to_remove = players.iter().find(|x| x.peer_socket_address == *peer_addr);
	if let Some(player_to_remove) = player_to_remove {
    player_to_remove.save_to_disk();
    players.iter().for_each(|x| {
	    game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::PlayerInfoRemove::PACKET_ID, lib::packets::clientbound::play::PlayerInfoRemove {
	      uuids: vec![player_to_remove.uuid],
	    }.try_into().unwrap());

			game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, lib::packets::clientbound::play::RemoveEntities {
        entity_ids: vec![player_to_remove.entity_id]
      }.try_into().unwrap());
    });
	}

  connections.remove(peer_addr);

  drop(connections);

  game.packet_send_queues.remove(peer_addr);
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
  pub packet_handler_actions: std::time::Duration,
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
  let players_clone = game.players.lock().unwrap().clone();
  let duration_clone_players = std::time::Instant::now() - now;


  let now = std::time::Instant::now();
  //prevents handling of two movements packets from one player during tick
  let mut moved_players: Vec<SocketAddr> = Vec::new();

  for packet_handler_action in game.packet_handler_actions.lock().unwrap().iter().rev() {
    match packet_handler_action {
      PacketHandlerAction::DisconnectPlayer(peer_addr) => {
        println!("handler told us to disconnect");
        disconnect_player(peer_addr, game.clone());
      },
      PacketHandlerAction::MovePlayer(peer_addr, position, rotation) => {
        if moved_players.contains(peer_addr) {
          continue;
        }

        moved_players.push(*peer_addr);

        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == *peer_addr) else { continue; };

        let player_entity_id = player.entity_id;
        let old_position = player.get_position();

        let position_updated = position.is_some();
        let rotation_updated = rotation.is_some();

        let packets: Vec<(u8, Vec<u8>)> = if position_updated && rotation_updated {
          let new_position = player.new_position_and_rotation(EntityPosition { x: position.unwrap().0, y: position.unwrap().1, z: position.unwrap().2, yaw: rotation.unwrap().0, pitch: rotation.unwrap().1 }, &mut game.world.lock().unwrap(), &game.entity_id_manager, &game.block_state_data, game.clone()).unwrap();
          vec![
            (
              lib::packets::clientbound::play::UpdateEntityPositionAndRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityPositionAndRotation {
                entity_id: player_entity_id,
                delta_x: ((new_position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
                delta_y: ((new_position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
                delta_z: ((new_position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
                yaw: player.get_yaw_u8(),
                pitch: player.get_pitch_u8(),
                on_ground: true, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
              }.try_into().unwrap()
            ),
            (
              lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
       	        entity_id: player_entity_id,
       					head_yaw: player.get_yaw_u8(),
       	      }.try_into().unwrap()
            )
          ]
        } else if position_updated {
          let new_position = player.new_position(position.unwrap().0, position.unwrap().1, position.unwrap().2, &mut game.world.lock().unwrap(), &game.entity_id_manager, &game.block_state_data, game.clone()).unwrap();
          vec![(lib::packets::clientbound::play::UpdateEntityPosition::PACKET_ID, lib::packets::clientbound::play::UpdateEntityPosition {
            entity_id: player_entity_id,
            delta_x: ((new_position.x * 4096.0) - (old_position.x * 4096.0)) as i16,
            delta_y: ((new_position.y * 4096.0) - (old_position.y * 4096.0)) as i16,
            delta_z: ((new_position.z * 4096.0) - (old_position.z * 4096.0)) as i16,
            on_ground: true, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
          }.try_into().unwrap())]
        } else if rotation_updated {
          player.new_rotation(rotation.unwrap().0, rotation.unwrap().1);
          vec![
            (
              lib::packets::clientbound::play::UpdateEntityRotation::PACKET_ID, lib::packets::clientbound::play::UpdateEntityRotation {
                entity_id: player_entity_id,
                yaw: player.get_yaw_u8(),
                pitch: player.get_pitch_u8(),
                on_ground: true, //add proper check https://git.thetxt.io/thetxt/oxide/issues/22
              }.try_into().unwrap()
            ),
            (
              lib::packets::clientbound::play::SetHeadRotation::PACKET_ID, lib::packets::clientbound::play::SetHeadRotation {
       	        entity_id: player_entity_id,
       					head_yaw: player.get_yaw_u8(),
       	      }.try_into().unwrap()
            )
          ]
        } else {
          Vec::new()
        };

        for other_player in players_clone.iter() {
          if other_player.peer_socket_address != *peer_addr {
            for packet in &packets {
              game.send_packet(&other_player.peer_socket_address, packet.0, packet.1.clone());
            }
          }
        }
      },
      PacketHandlerAction::ConfirmTeleportation(peer_addr, teleport_id) => {
        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == *peer_addr) else { continue; };

        if player.current_teleport_id == *teleport_id {
       		player.waiting_for_confirm_teleportation = false;
        }
      }
    }
  };
  *game.packet_handler_actions.lock().unwrap() = Vec::new();
  let duration_packet_handler_actions = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  if std::time::Instant::now() > *game.last_player_keepalive_timestamp.lock().unwrap() + std::time::Duration::from_secs(5) {
    *game.last_player_keepalive_timestamp.lock().unwrap() = std::time::Instant::now();

    let players = players_clone.clone();
    let game = game.clone();
    std::thread::spawn(move || {
      for player in &players {
        let useless_buf_no_one_crates_about = &mut [0; 1];
        if player.connection_stream.peek(useless_buf_no_one_crates_about).is_err() {
          disconnect_player(&player.peer_socket_address, game.clone());
        }
        game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::ClientboundKeepAlive::PACKET_ID, lib::packets::clientbound::play::ClientboundKeepAlive {
          keep_alive_id: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        }.try_into().unwrap());
      }
    });
  }
  let duration_send_keepalives = std::time::Instant::now() - now;

  let now = std::time::Instant::now();
  for dimension in &mut game.world.lock().unwrap().dimensions {
    for chunk in &mut dimension.1.chunks {
      for blockentity in &mut chunk.block_entities {
        if blockentity.needs_ticking {
          blockentity.tick(&players_clone, game.clone());
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
      let outcome = entity.tick(dimension.1, &players_clone, &game.block_state_data, game.clone());
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

          for player in &players_clone {
            game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::EntityEvent::PACKET_ID, entity_event_packet.clone().try_into().unwrap());
          }
        },
        EntityTickOutcome::RemoveSelf => {
          let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
            entity_ids: vec![outcome.0],
          };

          for player in &players_clone {
            game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::RemoveEntities::PACKET_ID, remove_entities_packet.clone().try_into().unwrap());
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
    packet_handler_actions: duration_packet_handler_actions,
  }
}
