use crate::types::*;
use data::{blocks::*, inventory::Inventory};
use std::collections::HashMap;
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

pub use state::*;


#[derive(Debug, PartialEq)]
pub enum BlockInteractionResult {
  OverwriteBlocks(Vec<(u16, BlockPosition)>),
  OpenInventory(Inventory), //Proper enum somewhere for window types; find types here https://minecraft.wiki/w/Java_Edition_protocol/Inventory
  OpenSignEditor,
  Nothing,
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
