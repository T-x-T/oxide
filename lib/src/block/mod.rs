use crate::types::*;
use crate::packets::Packet;
use data::{blocks::*, inventory::Inventory};
use std::{collections::HashMap, error::Error};
mod state;

mod door;
mod trapdoor;
mod fencegate;
mod rotated_pillar;
mod barell;
mod chest;
mod trapped_chest;
mod ender_chest;
mod slab;
mod stair;

pub use state::*;


#[derive(Debug, PartialEq)]
pub enum BlockInteractionResult {
  OverwriteBlocks(Vec<(u16, BlockPosition)>),
  OpenInventory(Inventory), //Proper enum somewhere for window types; find types here https://minecraft.wiki/w/Java_Edition_protocol/Inventory
  OpenSignEditor,
  Nothing,
}

impl BlockInteractionResult {
  pub fn handle(self, dimension: &Dimension, position: BlockPosition, player: &mut Player, players: &[Player], block_id_at_location: u16) -> Result<Vec<(u16, BlockPosition)>, Box<dyn Error>> {
    match self {
      BlockInteractionResult::OverwriteBlocks(blocks) => Ok(blocks),
      BlockInteractionResult::OpenInventory(window_type) => {
        let Some(block_entity) = dimension.get_chunk_from_position(position).unwrap().try_get_block_entity(position) else {
          return Err(Box::new(crate::CustomError::BlockEntityNotFoundAtLocation(position)));
        };
        player.open_inventory(window_type, block_entity);

        players.iter()
         	.for_each(|x| {
     	      crate::utils::send_packet(&x.connection_stream, crate::packets::clientbound::play::BlockAction::PACKET_ID, crate::packets::clientbound::play::BlockAction {
    	      	location: position,
              action_id: 1,
              action_parameter: 1,
              block_type: block_id_at_location as i32,
     	      }.try_into().unwrap()).unwrap();
          });
        Ok(Vec::new())
      },
      BlockInteractionResult::OpenSignEditor => {
        crate::utils::send_packet(&player.connection_stream, crate::packets::clientbound::play::OpenSignEditor::PACKET_ID, crate::packets::clientbound::play::OpenSignEditor {
          location: position,
          is_front_text: true,
        }.try_into()?)?;
        Ok(Vec::new())
      },
      BlockInteractionResult::Nothing => Ok(Vec::new()),
    }
  }
}

pub fn interact_with_block_at(location: BlockPosition, block_id_at_location: u16, face: u8, block_states: &HashMap<String, data::blocks::Block>) -> BlockInteractionResult {
  let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location, block_states);

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
    Type::WallSign | Type::StandingSign | Type::WallHangingSign | Type::CeilingHangingSign => {
      BlockInteractionResult::OpenSignEditor
    }
    _ => BlockInteractionResult::Nothing,
  };
}
