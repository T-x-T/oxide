use std::collections::HashMap;

use data::blocks::*;

use crate::{CardinalDirection, Dimension, BlockPosition};

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, pitch: f32, dimension: &Dimension, position: BlockPosition, used_item_name: &str, cursor_position_x: f32, cursor_position_y: f32, cursor_position_z: f32, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  match block.block_type {
    Type::RotatedPillar => output.append(&mut super::rotated_pillar::get_block_state_id(face, cardinal_direction, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::Barrel => output.append(&mut super::barell::get_block_state_id(face, cardinal_direction, pitch, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::Chest => output.append(&mut super::chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
    Type::TrappedChest => output.append(&mut super::trapped_chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
    Type::EnderChest => output.append(&mut super::ender_chest::get_block_state_id(cardinal_direction, position, used_item_name, block_states)),
    Type::Door => output.append(&mut super::door::get_block_state_id(face, cardinal_direction, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::Trapdoor => output.append(&mut super::trapdoor::get_block_state_id(face, cardinal_direction, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::FenceGate => output.append(&mut super::fencegate::get_block_state_id(face, cardinal_direction, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::Slab => output.append(&mut super::slab::get_block_state_id(face, cursor_position_y, dimension, position, used_item_name, block_states)),
    Type::Stair => output.append(&mut super::stair::get_block_state_id(face, cardinal_direction, cursor_position_y, dimension, position, used_item_name, block_states)),
    Type::IronBars => {
      let block_ids_to_check: Vec<u16> = block_states.iter().filter(|x| x.0.ends_with("glass_pane") || x.0 == "minecraft:iron_bars").flat_map(|x| x.1.states.iter().map(|x| x.id)).collect();

      let north = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z - 1, ..position }).unwrap_or(0)) { IronBarsNorth::True } else { IronBarsNorth::False };
      let south = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z + 1, ..position }).unwrap_or(0)) { IronBarsSouth::True } else { IronBarsSouth::False };
      let east = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x + 1, ..position }).unwrap_or(0)) { IronBarsEast::True } else { IronBarsEast::False };
      let west = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x - 1, ..position }).unwrap_or(0)) { IronBarsWest::True } else { IronBarsWest::False };
      let water_logged = IronBarsWaterlogged::False;

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::IronBarsNorth(north.clone())) && x.properties.contains(&Property::IronBarsSouth(south.clone())) && x.properties.contains(&Property::IronBarsEast(east.clone())) && x.properties.contains(&Property::IronBarsWest(west.clone())) && x.properties.contains(&Property::IronBarsWaterlogged(water_logged.clone()))).unwrap().id, position));
    },
    Type::StainedGlassPane => {
      let block_ids_to_check: Vec<u16> = block_states.iter().filter(|x| x.0.ends_with("glass_pane") || x.0 == "minecraft:iron_bars").flat_map(|x| x.1.states.iter().map(|x| x.id)).collect();

      let north = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z - 1, ..position }).unwrap_or(0)) { StainedGlassPaneNorth::True } else { StainedGlassPaneNorth::False };
      let south = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z + 1, ..position }).unwrap_or(0)) { StainedGlassPaneSouth::True } else { StainedGlassPaneSouth::False };
      let east = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x + 1, ..position }).unwrap_or(0)) { StainedGlassPaneEast::True } else { StainedGlassPaneEast::False };
      let west = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x - 1, ..position }).unwrap_or(0)) { StainedGlassPaneWest::True } else { StainedGlassPaneWest::False };
      let water_logged = StainedGlassPaneWaterlogged::False;

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::StainedGlassPaneNorth(north.clone())) && x.properties.contains(&Property::StainedGlassPaneSouth(south.clone())) && x.properties.contains(&Property::StainedGlassPaneEast(east.clone())) && x.properties.contains(&Property::StainedGlassPaneWest(west.clone())) && x.properties.contains(&Property::StainedGlassPaneWaterlogged(water_logged.clone()))).unwrap().id, position));
    },
    _ => (),
  }

  if output.is_empty() {
    output.push((block.states.iter().find(|x| x.default).unwrap().id, position));
  }

  return output;
}
