use super::*;

pub fn get_block_state_id(dimension: &Dimension, position: BlockPosition, used_item_name: &str, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  let block_ids_to_check: Vec<u16> = block_states.iter().filter(|x| x.1.block_type == Type::Fence || x.1.block_type == Type::FenceGate).flat_map(|x| x.1.states.iter().map(|x| x.id)).collect();

  let north_block = dimension.get_block(BlockPosition { z: position.z - 1, ..position }).unwrap_or(0);
  let east_block = dimension.get_block(BlockPosition { x: position.x + 1, ..position }).unwrap_or(0);
  let south_block = dimension.get_block(BlockPosition { z: position.z + 1, ..position }).unwrap_or(0);
  let west_block = dimension.get_block(BlockPosition { x: position.x - 1, ..position }).unwrap_or(0);

  let north_block_states = data::blocks::get_block_state_from_block_state_id(north_block, block_states);
  let east_block_states = data::blocks::get_block_state_from_block_state_id(east_block, block_states);
  let south_block_states = data::blocks::get_block_state_from_block_state_id(south_block, block_states);
  let west_block_states = data::blocks::get_block_state_from_block_state_id(west_block, block_states);

  let north = if block_ids_to_check.contains(&north_block) && !north_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && !north_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::South)) { FenceNorth::True } else { FenceNorth::False };
  let east = if block_ids_to_check.contains(&east_block) && !east_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && !east_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::West)) { FenceEast::True } else { FenceEast::False };
  let south = if block_ids_to_check.contains(&south_block) && !south_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && !south_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::South)) { FenceSouth::True } else { FenceSouth::False };
  let west = if block_ids_to_check.contains(&west_block) && !west_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && !west_block_states.properties.contains(&Property::FenceGateFacing(FenceGateFacing::West)) { FenceWest::True } else { FenceWest::False };

  let water_logged = FenceWaterlogged::False;

  output.push((block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(north.clone())) && x.properties.contains(&Property::FenceSouth(south.clone())) && x.properties.contains(&Property::FenceEast(east.clone())) && x.properties.contains(&Property::FenceWest(west.clone())) && x.properties.contains(&Property::FenceWaterlogged(water_logged.clone()))).unwrap().id, position));
  return output;
}

pub fn update(position: BlockPosition, dimension: &Dimension, block_states: &HashMap<String, Block>) -> Option<u16> {
  let Ok(block_state_id) = dimension.get_block(position) else {
    return None;
  };

  let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id, block_states);

  let res = get_block_state_id(dimension, position, &block_name, block_states);

  let new_block_state_id = res.first().unwrap().0;

  if block_state_id == new_block_state_id {
    return None;
  } else {
    return Some(new_block_state_id);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  mod get_block_state_id {
    use super::*;

    #[test]
    fn no_connections() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(&dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn north_and_west_connected() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence", &block_states);

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::True)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place).unwrap();
      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::True)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::True)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::True)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(&dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn all_sides_connected() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence", &block_states);

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::True)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place).unwrap();
      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::True)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place).unwrap();
      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::True)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place).unwrap();
      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::True)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::True)) && x.properties.contains(&Property::FenceSouth(FenceSouth::True)) && x.properties.contains(&Property::FenceEast(FenceEast::True)) && x.properties.contains(&Property::FenceWest(FenceWest::True)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(&dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn connects_to_fence_gate() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence", &block_states);

      let fence_gate_block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);
      let block_state_id_to_place = fence_gate_block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::True)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(&dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn doesnt_connect_to_fence_gate_with_wrong_orientation() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence", &block_states);

      let fence_gate_block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);
      let block_state_id_to_place = fence_gate_block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceNorth(FenceNorth::False)) && x.properties.contains(&Property::FenceSouth(FenceSouth::False)) && x.properties.contains(&Property::FenceEast(FenceEast::False)) && x.properties.contains(&Property::FenceWest(FenceWest::False)) && x.properties.contains(&Property::FenceWaterlogged(FenceWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(&dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }
  }
}
