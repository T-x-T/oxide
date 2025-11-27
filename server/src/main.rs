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

  let mut packet_handler_actions: Vec<PacketHandlerAction> = Vec::new();
  packet_handler_actions.append(&mut game.packet_handler_actions.lock().unwrap());

  for packet_handler_action in packet_handler_actions.into_iter().rev() {
    match packet_handler_action {
      PacketHandlerAction::DisconnectPlayer(peer_addr) => {
        println!("handler told us to disconnect");
        disconnect_player(&peer_addr, game.clone());
      },
      PacketHandlerAction::MovePlayer(peer_addr, position, rotation) => {
        if moved_players.contains(&peer_addr) {
          continue;
        }

        moved_players.push(peer_addr);

        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == peer_addr) else { continue; };

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
          if other_player.peer_socket_address != peer_addr {
            for packet in &packets {
              game.send_packet(&other_player.peer_socket_address, packet.0, packet.1.clone());
            }
          }
        }
      },
      PacketHandlerAction::ConfirmTeleportation(peer_addr, teleport_id) => {
        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.peer_socket_address == peer_addr) else { continue; };

        if player.current_teleport_id == teleport_id {
       		player.waiting_for_confirm_teleportation = false;
        }
      },
      PacketHandlerAction::SetCreativeModeSlot(peer_addr, slot, item) => {
        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr) else { continue; };

        player.set_inventory_slot(slot, item, &players_clone, game.clone());
      },
      PacketHandlerAction::SetSelectedSlot(peer_addr, slot) => {
        let mut players = game.players.lock().unwrap();
        let Some(player) = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr) else { continue; };

        player.set_selected_slot(slot, &players_clone, game.clone());
      },
      PacketHandlerAction::PickItemFromBlock(peer_addr, location, _include_data) => {
       	let picked_block = game.world.lock().unwrap().dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();
        let picked_block_name = game.block_state_data.iter().find(|x| x.1.states.iter().any(|x| x.id == picked_block)).unwrap().0.clone();
        let item_id = data::items::get_items().get(&picked_block_name).unwrap_or(&data::items::Item {max_stack_size: 0, rarity: data::items::ItemRarity::Common, id:0, repair_cost:0}).id;

        let mut players = game.players.lock().unwrap();
        let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

        let new_slot_data = Slot {
     	   	item_count: 1,
     	    item_id,
     	    components_to_add: Vec::new(),
     	    components_to_remove: Vec::new(),
        };

        player.set_selected_inventory_slot(Some(new_slot_data), &players_clone, game.clone());
      },
      PacketHandlerAction::SwingArm(peer_addr, hand) => {
      	let entity_id = players_clone.iter().find(|x| x.peer_socket_address == peer_addr).unwrap().entity_id;

        for other_player in players_clone.iter() {
          if other_player.peer_socket_address != peer_addr {
            game.send_packet(&other_player.peer_socket_address, lib::packets::clientbound::play::EntityAnimation::PACKET_ID, lib::packets::clientbound::play::EntityAnimation {
         			entity_id,
          		animation: if hand == 0 { 0 } else { 3 },
           	}.try_into().unwrap());
          }
        }
      },
      PacketHandlerAction::BreakBlock(peer_addr, _status, location, _face, sequence_id) => {
        let mut players = game.players.lock().unwrap();
        let mut world = game.world.lock().unwrap();

        let old_block_id = world.dimensions.get("minecraft:overworld").unwrap().get_block(location).unwrap();
        let old_block = data::blocks::get_block_from_block_state_id(old_block_id, &game.block_state_data);

        //TODO: move to a update function or similar
        if old_block.block_type == data::blocks::Type::Door {
          let block_state = data::blocks::get_block_state_from_block_state_id(old_block_id, &game.block_state_data);
          let location: Option<BlockPosition> = if block_state.properties.iter().any(|x| x == &data::blocks::Property::DoorHalf(data::blocks::DoorHalf::Upper)) {
            Some(BlockPosition { y: location.y - 1, ..location })
          } else if block_state.properties.iter().any(|x| x == &data::blocks::Property::DoorHalf(data::blocks::DoorHalf::Lower)) {
            Some(BlockPosition { y: location.y + 1, ..location })
          } else {
            None
          };

          if let Some(location) = location {
            world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(location, 0, &game.block_state_data).unwrap();

            players.iter()
              .inspect(|x| {
                game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
                  location,
                  block_id: 0,
                }.try_into().unwrap());
              })
             	.filter(|x| x.peer_socket_address != peer_addr)
             	.for_each(|x| {
         	      game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::WorldEvent::PACKET_ID, lib::packets::clientbound::play::WorldEvent {
        	      	event: 2001,
        	      	location,
         	       	data: old_block_id as i32,
         	      }.try_into().unwrap());
              });
          }
        }


        let res = world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(location, 0, &game.block_state_data).unwrap();
        if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
          let block_entity = world.dimensions.get("minecraft:overworld").unwrap().get_chunk_from_position(location).unwrap().block_entities.iter().find(|x| x.position == location).unwrap();
          let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
          lib::blockentity::remove_block_entity(&block_entity, &game.entity_id_manager, &mut players, &mut world, game.clone());
        }

       	players.iter()
          .inspect(|x| {
            game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
              location,
              block_id: 0,
            }.try_into().unwrap());
          })
         	.filter(|x| x.peer_socket_address != peer_addr)
         	.for_each(|x| {
     	      game.send_packet(&x.peer_socket_address, lib::packets::clientbound::play::WorldEvent::PACKET_ID, lib::packets::clientbound::play::WorldEvent {
    	      	event: 2001,
    	      	location,
     	       	data: old_block_id as i32,
     	      }.try_into().unwrap());
          });

        game.send_packet(&peer_addr, lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID, lib::packets::clientbound::play::AcknowledgeBlockChange {
          sequence_id,
        }.try_into().unwrap());

        let blocks_to_update = [
          BlockPosition {x: location.x + 1, ..location},
          BlockPosition {x: location.x - 1, ..location},
          BlockPosition {y: location.y + 1, ..location},
          BlockPosition {y: location.y - 1, ..location},
          BlockPosition {z: location.z + 1, ..location},
          BlockPosition {z: location.z - 1, ..location},
        ];

        for block_to_update in blocks_to_update {
          let res = lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
          if let Some(new_block) = res {
            match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block, &game.block_state_data) {
              Ok(_) => {
                for player in players.iter() {
                  game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
                    location: block_to_update,
                    block_id: new_block as i32,
                  }.try_into().unwrap());
                }
              },
              Err(err) => {
                println!("couldn't place block because {err}");
                continue;
              },
            }
          };
        }
      },
      PacketHandlerAction::UseItemOn(peer_addr, _hand, location, face, cursor_position_x, cursor_position_y, cursor_position_z, _inside_block, _world_border_hit, sequence_id) => {
        let mut players = game.players.lock().unwrap();
        let mut world = game.world.lock().unwrap();

        let mut new_block_location = location;
        match face {
          0 => new_block_location.y -= 1,
          1 => new_block_location.y += 1,
          2 => new_block_location.z -= 1,
          3 => new_block_location.z += 1,
          4 => new_block_location.x -= 1,
          _ => new_block_location.x += 1,
        }

        let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();
        let player_get_looking_cardinal_direction = player.get_looking_cardinal_direction().clone();

        let dimension = world.dimensions.get("minecraft:overworld").unwrap();
        let block_id_at_location = dimension.get_block(location).unwrap_or_default();
        let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location, &game.block_state_data);

        let blocks_to_place: Vec<(u16, BlockPosition)> = if block_type_at_location.has_right_click_behavior() && !player.is_sneaking() {
          //Don't place block, because player right clicked something that does something when right clicked
          let block_interaction_result = lib::block::interact_with_block_at(location, block_id_at_location, face, &game.block_state_data);
          block_interaction_result.handle(dimension, location, player, &players_clone, block_id_at_location, game.clone()).unwrap()
        } else {
          //Let's go - we can place a block
          let used_item_id = player.get_held_item(true).unwrap_or(&Slot { item_count: 0, item_id: 0, components_to_add: Vec::new(), components_to_remove: Vec::new() }).item_id;
          let used_item_name = data::items::get_item_name_by_id(used_item_id);
          let pitch = player.get_pitch();

          if used_item_name.ends_with("spawn_egg") {
            let dimension = world.dimensions.get_mut("minecraft:overworld").unwrap();
            lib::create_and_spawn_entity_from_egg(&used_item_name, game.entity_id_manager.get_new(), new_block_location, dimension, &players, game.clone());
          }

          lib::block::get_block_state_id(face, player_get_looking_cardinal_direction, pitch, world.dimensions.get_mut("minecraft:overworld").unwrap(), new_block_location, &used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, &game.block_state_data)
        };

        let mut blocks_to_update: Vec<BlockPosition> = Vec::new();
        for block_to_place in &blocks_to_place {
          match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_place.1, block_to_place.0, &game.block_state_data) {
            Ok(res) => {
              let block = data::blocks::get_block_from_block_state_id(block_to_place.0, &game.block_state_data);
              //Logic to open sign editor when player placed a new sign, maybe move somewhere else or something idk
              if block.block_type == data::blocks::Type::WallSign || block.block_type == data::blocks::Type::StandingSign || block.block_type == data::blocks::Type::WallHangingSign || block.block_type == data::blocks::Type::CeilingHangingSign {
                game.send_packet(&peer_addr, lib::packets::clientbound::play::OpenSignEditor::PACKET_ID, lib::packets::clientbound::play::OpenSignEditor {
                  location: block_to_place.1,
                  is_front_text: true,
                }.try_into().unwrap());
              }
              #[allow(clippy::collapsible_if)]
              if res.is_some() && res.unwrap() == BlockOverwriteOutcome::DestroyBlockentity {
                if let Some(block_entity) = world.dimensions.get("minecraft:overworld").unwrap().get_chunk_from_position(location).unwrap().block_entities.iter().find(|x| x.position == location) {
                  let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
                  crate::blockentity::remove_block_entity(&block_entity, &game.entity_id_manager, &mut players, &mut world, game.clone());
                };
              }

              blocks_to_update.append(&mut vec![
                BlockPosition {x: block_to_place.1.x + 1, ..block_to_place.1},
                BlockPosition {x: block_to_place.1.x - 1, ..block_to_place.1},
                BlockPosition {y: block_to_place.1.y + 1, ..block_to_place.1},
                BlockPosition {y: block_to_place.1.y - 1, ..block_to_place.1},
                BlockPosition {z: block_to_place.1.z + 1, ..block_to_place.1},
                BlockPosition {z: block_to_place.1.z - 1, ..block_to_place.1},
              ]);
            },
            Err(err) => {
              println!("couldn't place block because {err}");
              continue;
            },
          };
        }

        blocks_to_update.sort();
        blocks_to_update.dedup();

        let mut updated_blocks: Vec<(u16, BlockPosition)> = Vec::new();
        for block_to_update in blocks_to_update {
          let res = lib::block::update(block_to_update, world.dimensions.get("minecraft:overworld").unwrap(), &game.block_state_data).unwrap();
          if let Some(new_block) = res {
            match world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(block_to_update, new_block, &game.block_state_data) {
              Ok(_) => {
                updated_blocks.push((new_block, block_to_update));
              },
              Err(err) => {
                println!("couldn't place block because {err}");
                continue;
              },
            }
          };
        }

        let all_changed_blocks: Vec<(u16, BlockPosition)> = vec![blocks_to_place, updated_blocks].into_iter().flatten().collect();

        for player in players.iter() {
          for block in &all_changed_blocks {
            game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::BlockUpdate::PACKET_ID, lib::packets::clientbound::play::BlockUpdate {
              location: block.1,
              block_id: block.0 as i32,
            }.try_into().unwrap());
          }
        }

        game.send_packet(&peer_addr, lib::packets::clientbound::play::AcknowledgeBlockChange::PACKET_ID, lib::packets::clientbound::play::AcknowledgeBlockChange {
          sequence_id,
        }.try_into().unwrap());
      },
      PacketHandlerAction::SendChatMessage(peer_addr, message, timestamp, salt) => {
        let mut players = game.players.lock().unwrap();

        let player = players.iter_mut().find(|x| x.connection_stream.peer_addr().unwrap() == peer_addr).unwrap();

        println!("<{}>: {}", player.display_name, message);

        let packet_to_send = lib::packets::clientbound::play::PlayerChatMessage {
          global_index: player.chat_message_index,
          sender: player.uuid,
          index: 0,
          message_signature_bytes: Vec::new(),
          message: message.clone(),
          timestamp,
          salt,
          signature_array: Vec::new(),
          unsigned_content: None,
          filter_type: 0,
          filter_type_bits: Vec::new(),
          chat_type: 1,
          sender_name: NbtTag::Root(vec![
        		NbtTag::TagCompound("click_event".to_string(), vec![
         			NbtTag::String("action".to_string(), "suggest_command".to_string()),
         			NbtTag::String("command".to_string(), format!("/tell {}", player.display_name).to_string()),
         	]),
         	NbtTag::String("insertion".to_string(), player.display_name.clone()),
         	NbtTag::String("text".to_string(), player.display_name.clone()),
          ]),
          target_name: None,
        };

        for player in players.iter_mut() {
          player.chat_message_index += 1;
          game.send_packet(&player.peer_socket_address, lib::packets::clientbound::play::PlayerChatMessage::PACKET_ID, packet_to_send.clone().try_into().unwrap());
        }
      },
      PacketHandlerAction::SendCommand(peer_addr, command_string) => {
        let players = game.players.lock().unwrap();
        let player = players.iter().find(|x| x.peer_socket_address == peer_addr).unwrap();
        println!("<{}> invoked: {}", player.display_name, command_string);

        let commands = game.commands.lock().unwrap().clone();

       	let Some(command) = commands.iter().find(|x| x.name == command_string.split(" ").next().unwrap_or_default()) else {
      		game.send_packet(&peer_addr, lib::packets::clientbound::play::SystemChatMessage::PACKET_ID, lib::packets::clientbound::play::SystemChatMessage {
  				  content: NbtTag::Root(vec![
   					NbtTag::String("type".to_string(), "text".to_string()),
   					NbtTag::String("text".to_string(), "command not found".to_string()),
    				]),
  				  overlay: false,
          }.try_into().unwrap());
          continue;
        };

        let mut stream = player.connection_stream.try_clone().unwrap();
        drop(players);
        (command.execute)(command_string, Some(&mut stream), game.clone()).unwrap();
      },
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
