use std::collections::HashMap;

use data::blocks::{self, *};

use crate::{CardinalDirection, Dimension, BlockPosition};

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, dimension: &Dimension, position: BlockPosition, used_item_name: &str, cursor_position_x: f32, cursor_position_y: f32, cursor_position_z: f32, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  match block.block_type {
    Type::RotatedPillar => {
      let axis = match face {
        0|1 => RotatedPillarAxis::Y,
        2|3 => RotatedPillarAxis::Z,
        _ => RotatedPillarAxis::X,
      };
      output.push((block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(axis.clone()))).unwrap().id, position));
    },
    Type::Barrel => { //This should use the orientation the player is looking in, instead of the face that was placed against to match vanilla
      let axis = match face {
        0 => BarrelFacing::Down,
        1 => BarrelFacing::Up,
        2 => BarrelFacing::North,
        3 => BarrelFacing::South,
        4 => BarrelFacing::West,
        _ => BarrelFacing::East,
      };
      output.push((block.states.iter().find(|x|
        x.properties.contains(&Property::BarrelFacing(axis.clone())) &&
        x.properties.contains(&Property::BarrelOpen(BarrelOpen::False))
      ).unwrap().id, position));
    },
    Type::Chest => {
      let facing = match cardinal_direction {
        CardinalDirection::North => ChestFacing::South,
        CardinalDirection::East => ChestFacing::West,
        CardinalDirection::South => ChestFacing::North,
        CardinalDirection::West => ChestFacing::East,
      };
      output.push((block.states.iter().find(|x|
        x.properties.contains(&Property::ChestFacing(facing.clone())) &&
        x.properties.contains(&Property::ChestType(ChestType::Single)) &&
        x.properties.contains(&Property::ChestWaterlogged(ChestWaterlogged::False))
      ).unwrap().id, position));
    },
    Type::TrappedChest => {
      let facing = match cardinal_direction {
        CardinalDirection::North => TrappedChestFacing::South,
        CardinalDirection::East => TrappedChestFacing::West,
        CardinalDirection::South => TrappedChestFacing::North,
        CardinalDirection::West => TrappedChestFacing::East,
      };
      output.push((block.states.iter().find(|x|
        x.properties.contains(&Property::TrappedChestFacing(facing.clone())) &&
        x.properties.contains(&Property::TrappedChestType(TrappedChestType::Single)) &&
        x.properties.contains(&Property::TrappedChestWaterlogged(TrappedChestWaterlogged::False))
      ).unwrap().id, position));
    },
    Type::EnderChest => {
      let facing = match cardinal_direction {
        CardinalDirection::North => EnderChestFacing::South,
        CardinalDirection::East => EnderChestFacing::West,
        CardinalDirection::South => EnderChestFacing::North,
        CardinalDirection::West => EnderChestFacing::East,
      };
      output.push((block.states.iter().find(|x|
        x.properties.contains(&Property::EnderChestFacing(facing.clone())) &&
        x.properties.contains(&Property::EnderChestWaterlogged(EnderChestWaterlogged::False))
      ).unwrap().id, position));
    },
    Type::Door => output.append(&mut super::door::get_block_state_id(face, cardinal_direction, dimension, position, used_item_name, cursor_position_x, cursor_position_y, cursor_position_z, block_states)),
    Type::Slab => {
      let position_to_check = if face == 0 {
        BlockPosition { y: position.y + 1, ..position }
      } else if face == 1 {
        BlockPosition { y: position.y - 1, ..position }
      } else {
        position
      };
      let block_id_at_position_to_check = dimension.get_block(position_to_check).unwrap_or(0);
      let block_id_at_position = dimension.get_block(position).unwrap_or(0);

      let block_ids_of_half_slabs: Vec<u16> = block.states.iter().filter(|x| !x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();
      let block_ids_of_top_slabs: Vec<u16> = block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Top))).map(|x| x.id).collect();
      let block_ids_of_bottom_slabs: Vec<u16> = block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom))).map(|x| x.id).collect();
      let block_ids_of_double_slabs: Vec<u16> = block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();

      let placed_underneath_bottom_slab = block_ids_of_bottom_slabs.contains(&block_id_at_position_to_check) && face == 0;
      let double_up_placed_underneath_bottom_slab = placed_underneath_bottom_slab && block_ids_of_bottom_slabs.contains(&block_id_at_position);
      let placed_ontop_top_slab = block_ids_of_top_slabs.contains(&block_id_at_position_to_check) && face == 1;
      let double_up_placed_ontop_top_slab = placed_ontop_top_slab && block_ids_of_top_slabs.contains(&block_id_at_position);
      let double_it_up = (block_ids_of_half_slabs.contains(&block_id_at_position_to_check) && !placed_underneath_bottom_slab && !placed_ontop_top_slab) || double_up_placed_underneath_bottom_slab || double_up_placed_ontop_top_slab;

      let place_top = (face >= 2 && cursor_position_y > 0.5) || face == 0;
      let slab_type_to_place = if double_it_up { SlabType::Double } else if place_top { SlabType::Top } else { SlabType::Bottom };

      let its_already_doubled = block_ids_of_double_slabs.contains(&block_id_at_position_to_check);
      let position_to_place = if its_already_doubled { position } else if double_it_up { position_to_check } else { position };

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::SlabType(slab_type_to_place.clone())) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id, position_to_place));
    },
    Type::Stair => {
      let facing = match cardinal_direction {
        CardinalDirection::North => StairFacing::North,
        CardinalDirection::East => StairFacing::East,
        CardinalDirection::South => StairFacing::South,
        CardinalDirection::West => StairFacing::West,
      };

      let flip_it = face == 0 || (cursor_position_y > 0.5 && cursor_position_y < 0.9999);
      let stair_half = if flip_it { StairHalf::Top } else { StairHalf::Bottom };

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(facing.clone())) && x.properties.contains(&Property::StairHalf(stair_half.clone())) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id, position));
    },
    Type::IronBars => {
      let all_blocks = blocks::get_blocks();
      let block_ids_to_check: Vec<u16> = all_blocks.iter().filter(|x| x.0.ends_with("glass_pane") || x.0 == "minecraft:iron_bars").flat_map(|x| x.1.states.iter().map(|x| x.id)).collect();

      let north = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z - 1, ..position }).unwrap_or(0)) { IronBarsNorth::True } else { IronBarsNorth::False };
      let south = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { z: position.z + 1, ..position }).unwrap_or(0)) { IronBarsSouth::True } else { IronBarsSouth::False };
      let east = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x + 1, ..position }).unwrap_or(0)) { IronBarsEast::True } else { IronBarsEast::False };
      let west = if block_ids_to_check.contains(&dimension.get_block(BlockPosition { x: position.x - 1, ..position }).unwrap_or(0)) { IronBarsWest::True } else { IronBarsWest::False };
      let water_logged = IronBarsWaterlogged::False;

      output.push((block.states.iter().find(|x| x.properties.contains(&Property::IronBarsNorth(north.clone())) && x.properties.contains(&Property::IronBarsSouth(south.clone())) && x.properties.contains(&Property::IronBarsEast(east.clone())) && x.properties.contains(&Property::IronBarsWest(west.clone())) && x.properties.contains(&Property::IronBarsWaterlogged(water_logged.clone()))).unwrap().id, position));
    },
    Type::StainedGlassPane => {
      let all_blocks = blocks::get_blocks();
      let block_ids_to_check: Vec<u16> = all_blocks.iter().filter(|x| x.0.ends_with("glass_pane") || x.0 == "minecraft:iron_bars").flat_map(|x| x.1.states.iter().map(|x| x.id)).collect();

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
