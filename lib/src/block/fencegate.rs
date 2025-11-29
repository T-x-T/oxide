use super::*;

pub fn interact(location: BlockPosition, block_id_at_location: u16, face: u8, block_states: &HashMap<String, Block>) -> BlockInteractionResult {
  let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(block_states, block_id_at_location);
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


  let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, block_states);
  let new_block_id = data::blocks::get_block_state_id_from_raw(block_states, &block_name, &block_properties);

  return BlockInteractionResult::OverwriteBlocks(vec![(new_block_id, location)]);
}

pub fn get_block_state_id(_face: u8, cardinal_direction: CardinalDirection, _dimension: &Dimension, position: BlockPosition, used_item_name: &str, _cursor_position_x: f32, _cursor_position_y: f32, _cursor_position_z: f32, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  let facing = match cardinal_direction {
    CardinalDirection::North => FenceGateFacing::North,
    CardinalDirection::East => FenceGateFacing::East,
    CardinalDirection::South => FenceGateFacing::South,
    CardinalDirection::West => FenceGateFacing::West,
  };

  output.push((block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(facing.clone())) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id, position));

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  mod interact {
    use super::*;

    #[test]
    fn open_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_closed, 3, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_opened, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }

    #[test]
    fn open_south() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::South)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_closed, 2, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_opened, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }

    #[test]
    fn open_east() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_closed, 4, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_opened, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }

    #[test]
    fn open_west() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::West)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_closed, 5, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_opened, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }

    #[test]
    fn open_wrong_side() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::West)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_closed, 1, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_opened, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }


    #[test]
    fn close() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id_closed = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;
      let block_state_id_opened = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::True)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_opened, 3, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_closed, BlockPosition { x: 100, y: 80, z: -100 })
      ]);

      assert_eq!(res, expected);
    }
  }

  mod get_block_state_id {
    use super::*;

    #[test]
    fn north() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::North)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = get_block_state_id(0, CardinalDirection::North, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence_gate", 0.0, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn east() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_fence_gate", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::FenceGateFacing(FenceGateFacing::East)) && x.properties.contains(&Property::FenceGateInWall(FenceGateInWall::False)) && x.properties.contains(&Property::FenceGateOpen(FenceGateOpen::False)) && x.properties.contains(&Property::FenceGatePowered(FenceGatePowered::False))).unwrap().id;

      let res = get_block_state_id(0, CardinalDirection::East, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_fence_gate", 0.0, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }
  }
}
