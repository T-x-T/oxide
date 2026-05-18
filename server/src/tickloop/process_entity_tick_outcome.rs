use super::*;
use lib::entity::CommonEntity;
use lib::loot_table;
use lib::packets::Packet;

pub fn process(entity_tick_outcomes: Vec<(i32, EntityTickOutcome)>, game: Arc<Game>, players_clone: &[Player], dimension: &mut Dimension) {
	let mut players = game.players.lock().unwrap();

	let mut already_bred_pairs: Vec<(i32, i32)> = Vec::new();
	for (entity_id, outcome) in entity_tick_outcomes {
		match outcome {
			EntityTickOutcome::SelfDied => {
				let entity_event_packet = lib::packets::clientbound::play::EntityEvent {
					entity_id,
					entity_status: 3,
				};

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::EntityEvent::PACKET_ID,
					entity_event_packet,
				);

				let entity_clone: Entity = dimension.entities.iter().find(|x| x.get_common_entity_data().entity_id == entity_id).unwrap().clone();

				let items_to_drop = loot_table::get_entity_drops(
					&game.loot_tables,
					&data::entities::get_name_from_id(entity_clone.get_type()),
					&Slot::default(),
					&game.block_state_data,
					None,
				);

				for item_to_drop in items_to_drop {
					dimension.summon_item(
						entity_clone.get_common_entity_data().position,
						item_to_drop,
						None,
						players_clone,
						&game.packet_sender,
						&game.entity_id_manager,
					);
				}
			}
			//Currently unused, might not be needed after all?
			EntityTickOutcome::KilledBy(entity_clone) => {
				let entity_event_packet = lib::packets::clientbound::play::EntityEvent {
					entity_id,
					entity_status: 3,
				};

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::EntityEvent::PACKET_ID,
					entity_event_packet,
				);

				let mut used_tool: Slot = Slot::default();
				let mut cloned_player: Option<Entity> = None;
				if let Some(player) = players.iter_mut().find(|x| x.entity_id == entity_id) {
					used_tool = player.get_held_item(true).cloned().unwrap_or_default();
					cloned_player = Some(Entity::Player(player.clone()));
				};

				let items_to_drop = loot_table::get_entity_drops(
					&game.loot_tables,
					&data::entities::get_name_from_id(entity_clone.get_type()),
					&used_tool,
					&game.block_state_data,
					cloned_player.as_ref(),
				);

				for item_to_drop in items_to_drop {
					dimension.summon_item(
						entity_clone.get_common_entity_data().position,
						item_to_drop,
						None,
						players_clone,
						&game.packet_sender,
						&game.entity_id_manager,
					);
				}
			}
			EntityTickOutcome::RemoveSelf => {
				let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
					entity_ids: vec![entity_id],
				};

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::RemoveEntities::PACKET_ID,
					remove_entities_packet,
				);

				if let Some(chunk) = dimension.get_chunk_from_position_mut(
					dimension
						.entities
						.iter()
						.find(|x| x.get_common_entity_data().entity_id == entity_id)
						.unwrap()
						.get_common_entity_data()
						.position
						.into(),
				) {
					chunk.modified = true;
				};
				dimension.entities.retain(|x| x.get_common_entity_data().entity_id != entity_id);
			}
			EntityTickOutcome::Updated => {
				if let Some(chunk) = dimension.get_chunk_from_position_mut(
					dimension
						.entities
						.iter()
						.find(|x| x.get_common_entity_data().entity_id == entity_id)
						.unwrap()
						.get_common_entity_data()
						.position
						.into(),
				) {
					chunk.modified = true;
				};
			}
			EntityTickOutcome::RemoveOthers(entity_ids) => {
				let remove_entities_packet = lib::packets::clientbound::play::RemoveEntities {
					entity_ids: entity_ids.clone(),
				};

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::RemoveEntities::PACKET_ID,
					remove_entities_packet,
				);

				dimension.entities.retain(|x| !entity_ids.contains(&x.get_common_entity_data().entity_id));
			}
			EntityTickOutcome::DamageSelf(damage) => {
				if let Some(entity) = dimension.entities.iter_mut().find(|x| x.get_common_entity_data().entity_id == entity_id) {
					entity.damage(damage, &game.packet_sender, players_clone);
				};
				if let Some(player) = players.iter_mut().find(|x| x.entity_id == entity_id) {
					player.damage(damage, &game.packet_sender, players_clone);
				};
			}
			EntityTickOutcome::SummonEntity(entity) => {
				let spawn_packet = entity.to_spawn_entity_packet();

				let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
					entity_id: entity.get_common_entity_data().entity_id,
					metadata: entity.get_metadata(),
				};

				dimension.add_entity(*entity);

				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
					spawn_packet,
				);
				game.packet_sender.send_packet_to_everyone_in_dimension(
					players_clone,
					&dimension.name,
					lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
					metadata_packet,
				);
			}
			EntityTickOutcome::DoneBreeding(a, b) => {
				if !already_bred_pairs.contains(&(a, b)) && !already_bred_pairs.contains(&(b, a)) {
					already_bred_pairs.push((a, b));
					let entity_a = dimension.entities.iter().find(|x| x.get_common_entity_data().entity_id == a).unwrap();
					let entity_b = dimension.entities.iter().find(|x| x.get_common_entity_data().entity_id == b).unwrap();

					if entity_a.get_type() != entity_b.get_type() {
						println!("bred two entities of different types!!\na: {entity_a:?}\nb: {entity_b:?}");
						continue;
					}

					let mut entity = entity::new(
						&data::entities::get_name_from_id(entity_a.get_type()),
						CommonEntity {
							position: entity_a.get_common_entity_data().position,
							velocity: EntityPosition::default(),
							uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
							entity_id: game.entity_id_manager.get_new(),
							..Default::default()
						},
						NbtListTag::TagCompound(Vec::new()),
					)
					.unwrap();

					entity.set_age(-lib::MOB_GROW_UP_TICKS);

					let spawn_packet = entity.to_spawn_entity_packet();

					let metadata_packet = lib::packets::clientbound::play::SetEntityMetadata {
						entity_id: entity.get_common_entity_data().entity_id,
						metadata: entity.get_metadata(),
					};

					dimension.add_entity(entity);

					game.packet_sender.send_packet_to_everyone_in_dimension(
						players_clone,
						&dimension.name,
						lib::packets::clientbound::play::SpawnEntity::PACKET_ID,
						spawn_packet,
					);
					game.packet_sender.send_packet_to_everyone_in_dimension(
						players_clone,
						&dimension.name,
						lib::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
						metadata_packet,
					);
				}
			}
			EntityTickOutcome::ReplaceBlock(block_position, block_state_id) => {
				if let Ok(res) = dimension.overwrite_block(block_position, block_state_id) {
					#[allow(clippy::collapsible_if)]
					if res.is_some() && res.unwrap() == BlockOverwriteOutcome::DestroyBlockentity {
						if let Some(block_entity) =
							dimension.get_chunk_from_position(block_position).unwrap().block_entities.iter().find(|x| x.get_position() == block_position)
						{
							let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
							block_entity.remove_self(&mut players, dimension, &game.packet_sender, &game.entity_id_manager);
						};
					}

					lib::block::update_all_recursively(
						dimension,
						block_position,
						&mut players,
						&game.packet_sender,
						&game.entity_id_manager,
						&game.block_state_data,
						&game.loot_tables,
					);

					game.packet_sender.send_packet_to_everyone_in_dimension(
						players_clone,
						&dimension.name,
						lib::packets::clientbound::play::BlockUpdate::PACKET_ID,
						lib::packets::clientbound::play::BlockUpdate {
							location: block_position,
							block_id: block_state_id as i32,
						},
					);
				};
			}
			EntityTickOutcome::UseNetherPortal(new_dimension_name) => {
				let mut entities = std::mem::take(&mut dimension.entities);
				if let Some(_entity) = entities.iter_mut().find(|x| x.get_common_entity_data().entity_id == entity_id) {
					todo!();
					//dont forget that entity needs to be put in the other dimension
				}

				if let Some(player) = players.iter_mut().find(|x| x.entity_id == entity_id) {
					game.task_queue.insert(Task::PlayerUseNetherPortal(player.uuid, new_dimension_name));
				}

				dimension.entities = entities;
			}
			EntityTickOutcome::UseEndPortal(new_dimension_name) => {
				let mut entities = std::mem::take(&mut dimension.entities);
				if let Some(_entity) = entities.iter_mut().find(|x| x.get_common_entity_data().entity_id == entity_id) {
					todo!();
					//dont forget that entity needs to be put in the other dimension
				}

				if let Some(player) = players.iter_mut().find(|x| x.entity_id == entity_id) {
					game.task_queue.insert(Task::PlayerUseEndPortal(player.uuid, new_dimension_name));
				}

				dimension.entities = entities;
			}
		}
	}
}
