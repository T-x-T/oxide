use crate::Position;
use data::blocks::Type;
use data::inventory::Inventory;

pub enum BlockInteractionResult {
  OverwriteBlocks(Vec<(u16, Position)>),
  OpenInventory(Inventory), //Proper enum somewhere for window types; find types here https://minecraft.wiki/w/Java_Edition_protocol/Inventory
  Nothing,
}

pub fn interact_with_block_at(location: Position, block_id_at_location: u16, face: u8) -> BlockInteractionResult {
  let block_states = data::blocks::get_blocks();
  let block_type_at_location = data::blocks::get_type_from_block_state_id(block_id_at_location, &block_states);

  return match block_type_at_location {
    Type::Door => {
      let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(&block_states, block_id_at_location);
      let is_open = block_properties.iter().find(|x| x.0 == "open").unwrap().1 == "true";
      block_properties.retain(|x| x.0 != "open");
      block_properties.push(("open".to_string(), if is_open { "false".to_string() } else { "true".to_string() }));

      let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, &block_states);
      let new_block_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties.clone());

      let is_upper = block_properties.iter().find(|x| x.0 == "half").unwrap().1 == "upper";
      block_properties.retain(|x| x.0 != "half");
      let other_half: (u16, Position) = if is_upper {
        block_properties.push(("half".to_string(), "lower".to_string()));
        let other_half_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties);
        let other_half_location = Position { y: location.y - 1, ..location};
        (other_half_id, other_half_location)
      } else {
        block_properties.push(("half".to_string(), "upper".to_string()));
        let other_half_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties);
        let other_half_location = Position { y: location.y + 1, ..location};
        (other_half_id, other_half_location)
      };

      BlockInteractionResult::OverwriteBlocks(vec![(new_block_id, location), other_half])
    },
    Type::Trapdoor => {
      let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(&block_states, block_id_at_location);
      let is_open = block_properties.iter().find(|x| x.0 == "open").unwrap().1 == "true";
      block_properties.retain(|x| x.0 != "open");
      block_properties.push(("open".to_string(), if is_open { "false".to_string() } else { "true".to_string() }));

      let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, &block_states);
      let new_block_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties.clone());

      BlockInteractionResult::OverwriteBlocks(vec![(new_block_id, location)])
    },
    Type::FenceGate => {
      let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(&block_states, block_id_at_location);
      let is_open = block_properties.iter().find(|x| x.0 == "open").unwrap().1 == "true";
      block_properties.retain(|x| x.0 != "open");
      block_properties.push(("open".to_string(), if is_open { "false".to_string() } else { "true".to_string() }));
      let facing = block_properties.iter().find(|x| x.0 == "facing").unwrap().1.clone();
      block_properties.retain(|x| x.0 != "facing");
      if facing == "north" || facing == "south" {
        if face == 2 {
          block_properties.push(("facing".to_string(), "south".to_string()));
        } else {
          block_properties.push(("facing".to_string(), "north".to_string()));
        }
      } else if face == 4 {
        block_properties.push(("facing".to_string(), "east".to_string()));
      } else {
        block_properties.push(("facing".to_string(), "west".to_string()));
      }


      let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, &block_states);
      let new_block_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties.clone());

      BlockInteractionResult::OverwriteBlocks(vec![(new_block_id, location)])
    },
    Type::CraftingTable => BlockInteractionResult::OpenInventory(Inventory::Crafting),
    Type::Chest => BlockInteractionResult::OpenInventory(Inventory::Generic9x3),
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
    _ => BlockInteractionResult::Nothing,
  };
}
