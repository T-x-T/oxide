use crate::packets::Packet;
use crate::types::*;
use data::blocks::*;
use data::inventory::Inventory;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

mod barell;
mod chest;
mod door;
mod ender_chest;
mod fence;
mod fencegate;
mod iron_bars;
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
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
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
		output.push((block.states.iter().find(|x| x.default).unwrap().id, position));
	}

	return output;
}

pub fn update(
	position: BlockPosition,
	dimension: &Dimension,
	block_states: &HashMap<String, Block>,
) -> Result<Option<u16>, Box<dyn Error>> {
	let block_state_id = dimension.get_block(position)?;
	let block_type = data::blocks::get_type_from_block_state_id(block_state_id);

	let res = match block_type {
		Type::Stair => stair::update(position, dimension, block_states),
		Type::IronBars => iron_bars::update(position, dimension, block_states),
		Type::StainedGlassPane => stained_glass_pane::update(position, dimension, block_states),
		Type::Fence => fence::update(position, dimension, block_states),
		_ => None,
	};

	return Ok(res);
}


#[derive(Debug, PartialEq)]
pub enum BlockInteractionResult {
	OverwriteBlocks(Vec<(u16, BlockPosition)>),
	OpenInventory(Inventory), //Proper enum somewhere for window types; find types here https://minecraft.wiki/w/Java_Edition_protocol/Inventory
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
				let Some(block_entity) = dimension.get_chunk_from_position(position).unwrap().try_get_block_entity(position) else {
					return Err(Box::new(crate::CustomError::BlockEntityNotFoundAtLocation(position)));
				};
				player.open_inventory(window_type, block_entity, game.clone());

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
					crate::packets::clientbound::play::OpenSignEditor { location: position, is_front_text: true }.try_into()?,
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
	block_states: &HashMap<String, data::blocks::Block>,
) -> BlockInteractionResult {
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
