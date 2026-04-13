use crate::packets::Packet;
use crate::types::*;
use basic_types::blocks::*;
use data::inventory::Inventory;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

mod barell;
mod beetroot;
#[allow(clippy::module_inception)]
mod block;
mod carrot;
mod chest;
mod crop;
mod door;
mod ender_chest;
mod farm;
mod fence;
mod fencegate;
mod iron_bars;
mod potato;
mod rotated_pillar;
mod slab;
mod stained_glass_pane;
mod stair;
mod trapdoor;
mod trapped_chest;


pub fn get_block_state_id(
	face: u8,
	cardinal_direction: CardinalDirection,
	pitch: f32,
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	cursor_position_x: f32,
	cursor_position_y: f32,
	cursor_position_z: f32,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block_name = match used_item_name {
		"minecraft:wheat_seeds" => "minecraft:wheat",
		"minecraft:carrot" => "minecraft:carrots",
		"minecraft:potato" => "minecraft:potatoes",
		"minecraft:beetroot_seeds" => "minecraft:beetroots",
		_ => used_item_name,
	};
	let block = data::blocks::get_block_from_name(block_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	match block.block_type {
		Type::RotatedPillar => output.append(&mut rotated_pillar::get_block_state_id(
			face,
			cardinal_direction,
			dimension,
			position,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			block_states,
		)),
		Type::Barrel => output.append(&mut barell::get_block_state_id(
			face,
			cardinal_direction,
			pitch,
			dimension,
			position,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			block_states,
		)),
		Type::Chest => output.append(&mut chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
		Type::TrappedChest => output.append(&mut trapped_chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
		Type::EnderChest => output.append(&mut ender_chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
		Type::Door => output.append(&mut door::get_block_state_id(
			face,
			cardinal_direction,
			dimension,
			position,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			block_states,
		)),
		Type::Trapdoor => output.append(&mut trapdoor::get_block_state_id(
			face,
			cardinal_direction,
			dimension,
			position,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			block_states,
		)),
		Type::FenceGate => output.append(&mut fencegate::get_block_state_id(
			face,
			cardinal_direction,
			dimension,
			position,
			used_item_name,
			cursor_position_x,
			cursor_position_y,
			cursor_position_z,
			block_states,
		)),
		Type::Slab => output.append(&mut slab::get_block_state_id(face, cursor_position_y, dimension, position, used_item_name, block_states)),
		Type::Stair => output.append(&mut stair::get_block_state_id(
			face,
			cardinal_direction,
			cursor_position_y,
			dimension,
			position,
			used_item_name,
			block_states,
		)),
		Type::IronBars => output.append(&mut iron_bars::get_block_state_id(dimension, position, used_item_name, block_states)),
		Type::StainedGlassPane => output.append(&mut stained_glass_pane::get_block_state_id(dimension, position, used_item_name, block_states)),
		Type::Fence => output.append(&mut fence::get_block_state_id(dimension, position, used_item_name, block_states)),
		_ => (),
	}

	if output.is_empty() {
		output.push((block.states[block.default_state].id, position));
	}

	return output;
}

#[derive(Debug)]
pub enum BlockUpdateOutcome {
	DoNothing,
	ChangeOwnBlockId(u16),
	DestroyAndDropSelf(u16),
}

impl BlockUpdateOutcome {
	pub fn handle(self, world: &mut World, position: BlockPosition, players: &mut [Player], game: Arc<Game>) {
		match self {
			BlockUpdateOutcome::DoNothing => return,
			BlockUpdateOutcome::ChangeOwnBlockId(new_block_id) => {
				let res = world.dimensions.get_mut("minecraft:overworld").unwrap().overwrite_block(position, new_block_id).unwrap();

				if res.is_some() && matches!(res.unwrap(), BlockOverwriteOutcome::DestroyBlockentity) {
					let block_entity = world
						.dimensions
						.get("minecraft:overworld")
						.unwrap()
						.get_chunk_from_position(position)
						.unwrap()
						.block_entities
						.iter()
						.find(|x| x.get_position() == position)
						.unwrap();
					let block_entity = block_entity.clone(); //So we get rid of the immutable borrow, so we can borrow world mutably again
					block_entity.remove_self(&game.entity_id_manager, players, world, game.clone());
				}

				for player in players.iter() {
					game.send_packet(
						&player.peer_socket_address,
						crate::packets::clientbound::play::BlockUpdate::PACKET_ID,
						crate::packets::clientbound::play::BlockUpdate {
							location: position,
							block_id: new_block_id as i32,
						}
						.try_into()
						.unwrap(),
					);
				}
			}
			BlockUpdateOutcome::DestroyAndDropSelf(old_block_id) => {
				for player in players.iter() {
					game.send_packet(
						&player.peer_socket_address,
						crate::packets::clientbound::play::BlockUpdate::PACKET_ID,
						crate::packets::clientbound::play::BlockUpdate {
							location: position,
							block_id: 0,
						}
						.try_into()
						.unwrap(),
					);
				}

				let items_to_drop = crate::loot_table::get_block_drops(&game.loot_tables, old_block_id, &Slot::default(), &game.block_state_data);

				for item_to_drop in items_to_drop {
					if item_to_drop.id != 0 {
						let new_entity = crate::entity::ItemEntity {
							common: crate::entity::CommonEntity {
								position: EntityPosition {
									x: position.x as f64 + 0.5,
									y: position.y as f64,
									z: position.z as f64 + 0.5,
									yaw: 0.0,
									pitch: 0.0,
								},
								velocity: EntityPosition::default(),
								uuid: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros(), //TODO: add proper UUID
								entity_id: game.entity_id_manager.get_new(),
								..Default::default()
							},
							age: 0,
							health: 5,
							item: item_to_drop,
							owner: 0,
							pickup_delay: 0,
							thrower: 0,
						};

						let packet = new_entity.to_spawn_entity_packet();

						let metadata_packet = crate::packets::clientbound::play::SetEntityMetadata {
							entity_id: new_entity.get_common_entity_data().entity_id,
							metadata: new_entity.get_metadata(),
						};

						world.dimensions.get_mut("minecraft:overworld").unwrap().add_entity(Entity::Item(new_entity));

						players.iter().for_each(|x| {
							game.send_packet(
								&x.peer_socket_address,
								crate::packets::clientbound::play::SpawnEntity::PACKET_ID,
								packet.clone().try_into().unwrap(),
							);
							game.send_packet(
								&x.peer_socket_address,
								crate::packets::clientbound::play::SetEntityMetadata::PACKET_ID,
								metadata_packet.clone().try_into().unwrap(),
							);
						});
					}
				}
			}
		}
	}
}

pub fn update(
	position: BlockPosition,
	dimension: &Dimension,
	block_states: &HashMap<String, Block>,
) -> Result<BlockUpdateOutcome, Box<dyn Error>> {
	let block_state_id = dimension.get_block(position)?;
	let block_type = data::blocks::get_type_from_block_state_id(block_state_id);

	let res = match block_type {
		Type::Stair => stair::update(position, dimension, block_states, block_state_id),
		Type::IronBars => iron_bars::update(position, dimension, block_states, block_state_id),
		Type::StainedGlassPane => stained_glass_pane::update(position, dimension, block_states, block_state_id),
		Type::Fence => fence::update(position, dimension, block_states, block_state_id),
		Type::Door => door::update(position, dimension, block_states, block_state_id),
		Type::Crop => crop::update(position, dimension, block_states, block_state_id),
		Type::Beetroot => beetroot::update(position, dimension, block_states, block_state_id),
		Type::Carrot => carrot::update(position, dimension, block_states, block_state_id),
		Type::Potato => potato::update(position, dimension, block_states, block_state_id),
		_ => BlockUpdateOutcome::DoNothing,
	};

	return Ok(res);
}


#[derive(Debug, PartialEq)]
pub enum BlockInteractionResult {
	OverwriteBlocks(Vec<(u16, BlockPosition)>),
	OpenInventory(Inventory),
	OpenSignEditor,
	Nothing,
}

impl BlockInteractionResult {
	pub fn handle(
		self,
		dimension: &Dimension,
		position: BlockPosition,
		player: &mut Player,
		players: &[Player],
		block_id_at_location: u16,
		game: Arc<Game>,
	) -> Result<Vec<(u16, BlockPosition)>, Box<dyn Error>> {
		match self {
			BlockInteractionResult::OverwriteBlocks(blocks) => Ok(blocks),
			BlockInteractionResult::OpenInventory(window_type) => {
				player.open_inventory(window_type, position, game.clone(), dimension);

				players.iter().for_each(|x| {
					game.send_packet(
						&x.peer_socket_address,
						crate::packets::clientbound::play::BlockAction::PACKET_ID,
						crate::packets::clientbound::play::BlockAction {
							location: position,
							action_id: 1,
							action_parameter: 1,
							block_type: block_id_at_location as i32,
						}
						.try_into()
						.unwrap(),
					);
				});
				Ok(Vec::new())
			}
			BlockInteractionResult::OpenSignEditor => {
				game.send_packet(
					&player.peer_socket_address,
					crate::packets::clientbound::play::OpenSignEditor::PACKET_ID,
					crate::packets::clientbound::play::OpenSignEditor {
						location: position,
						is_front_text: true,
					}
					.try_into()?,
				);
				Ok(Vec::new())
			}
			BlockInteractionResult::Nothing => Ok(Vec::new()),
		}
	}
}

pub fn interact_with_block_at(
	location: BlockPosition,
	block_id_at_location: u16,
	face: u8,
	block_states: &HashMap<String, Block>,
	used_tool: &Option<Slot>,
) -> BlockInteractionResult {
	let block_name_at_location = data::blocks::get_block_name_from_block_state_id(block_id_at_location, block_states);
	if ("minecraft:grass_block" == block_name_at_location || "minecraft:dirt" == block_name_at_location)
		&& data::tags::get_item().get("hoes").unwrap().contains(&data::items::get_item_name_by_id(used_tool.clone().unwrap_or_default().id))
	{
		let block = data::blocks::get_block_from_name("minecraft:farmland", block_states);
		let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num0))).unwrap().id;
		return BlockInteractionResult::OverwriteBlocks(vec![(block_state_id, location)]);
	}

	let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location);

	return match block_type_at_location {
		Type::Door => door::interact(location, block_id_at_location, face, block_states),
		Type::Trapdoor => trapdoor::interact(location, block_id_at_location, face, block_states),
		Type::FenceGate => fencegate::interact(location, block_id_at_location, face, block_states),
		Type::CraftingTable => BlockInteractionResult::OpenInventory(Inventory::Crafting),
		Type::Chest => BlockInteractionResult::OpenInventory(Inventory::Generic9x3),
		Type::TrappedChest => BlockInteractionResult::OpenInventory(Inventory::Generic9x3),
		Type::EnderChest => BlockInteractionResult::OpenInventory(Inventory::Generic9x3),
		Type::Barrel => BlockInteractionResult::OpenInventory(Inventory::Generic9x3),
		Type::Dispenser => BlockInteractionResult::OpenInventory(Inventory::Generic3x3),
		Type::Dropper => BlockInteractionResult::OpenInventory(Inventory::Generic3x3),
		Type::Crafter => BlockInteractionResult::OpenInventory(Inventory::Crafter3x3),
		Type::Anvil => BlockInteractionResult::OpenInventory(Inventory::Anvil),
		Type::Beacon => BlockInteractionResult::OpenInventory(Inventory::Beacon),
		Type::BlastFurnace => BlockInteractionResult::OpenInventory(Inventory::BlastFurnace),
		Type::BrewingStand => BlockInteractionResult::OpenInventory(Inventory::BrewingStand),
		Type::EnchantmentTable => BlockInteractionResult::OpenInventory(Inventory::Enchantment),
		Type::Furnace => BlockInteractionResult::OpenInventory(Inventory::Furnace),
		Type::Grindstone => BlockInteractionResult::OpenInventory(Inventory::Grindstone),
		Type::Hopper => BlockInteractionResult::OpenInventory(Inventory::Hopper),
		Type::Lectern => BlockInteractionResult::OpenInventory(Inventory::Lectern),
		Type::Loom => BlockInteractionResult::OpenInventory(Inventory::Loom),
		Type::ShulkerBox => BlockInteractionResult::OpenInventory(Inventory::ShulkerBox),
		Type::SmithingTable => BlockInteractionResult::OpenInventory(Inventory::Smithing),
		Type::Smoker => BlockInteractionResult::OpenInventory(Inventory::Smoker),
		Type::CartographyTable => BlockInteractionResult::OpenInventory(Inventory::CartographyTable),
		Type::Stonecutter => BlockInteractionResult::OpenInventory(Inventory::Stonecutter),
		Type::WallSign | Type::StandingSign | Type::WallHangingSign | Type::CeilingHangingSign => BlockInteractionResult::OpenSignEditor,
		_ => BlockInteractionResult::Nothing,
	};
}

pub fn get_hardness(block_id: u16, block_states: &HashMap<String, Block>) -> f32 {
	let block = data::blocks::get_block_from_block_state_id(block_id, block_states);

	return match block.block_type {
		Type::Barrel => 2.5,
		Type::Chest => 2.5,
		Type::Door => {
			if block.block_name == "minecraft:iron_door" {
				5.0
			} else {
				3.0
			}
		}
		Type::DropExperience => {
			if block.block_name.starts_with("deepslate") {
				4.5
			} else {
				3.0
			}
		}
		Type::EnderChest => 22.5,
		Type::Fence => 2.0,
		Type::FenceGate => 2.0,
		Type::IronBars => 5.0,
		Type::RotatedPillar => rotated_pillar::get_hardness(block_id, block, block_states),
		Type::Slab => slab::get_hardness(block_id, block, block_states),
		Type::StainedGlassPane => 0.3,
		Type::Stair => stair::get_hardness(block_id, block, block_states),
		Type::TallGrass => 0.0,
		Type::Trapdoor => trapdoor::get_hardness(block_id, block, block_states),
		Type::TrappedChest => 2.5,
		Type::Block => block::get_hardness(block_id, block, block_states),
		Type::TallDryGrass => 0.0,
		Type::DoublePlant => 0.0,
		Type::Crop => 0.0,
		Type::Carrot => 0.0,
		Type::Potato => 0.0,
		Type::Beetroot => 0.0,
		_ => 1.0,
	};
}

pub fn tick(
	current_block_state_id: u16,
	dimension: &Dimension,
	block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> u16 {
	let block_type = data::blocks::get_type_from_block_state_id(current_block_state_id);

	return match block_type {
		Type::Crop => crop::tick(current_block_state_id, dimension, block_position, block_states),
		Type::Carrot => carrot::tick(current_block_state_id, dimension, block_position, block_states),
		Type::Potato => potato::tick(current_block_state_id, dimension, block_position, block_states),
		Type::Beetroot => beetroot::tick(current_block_state_id, dimension, block_position, block_states),
		Type::Farm => farm::tick(current_block_state_id, dimension, block_position, block_states),
		_ => current_block_state_id,
	};
}
