use super::*;

pub fn interact(location: BlockPosition, block_id_at_location: u16, _face: u8, block_states: &HashMap<String, Block>) -> BlockInteractionResult {
  let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(block_states, block_id_at_location);
  let is_open = block_properties.iter().find(|x| x.0 == "open").unwrap().1 == "true";
  block_properties.retain(|x| x.0 != "open");
  block_properties.push(("open".to_string(), if is_open { "false".to_string() } else { "true".to_string() }));

  let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, block_states);
  let new_block_id = data::blocks::get_block_state_id_from_raw(block_states, &block_name, &block_properties);

  let is_upper = block_properties.iter().find(|x| x.0 == "half").unwrap().1 == "upper";
  block_properties.retain(|x| x.0 != "half");
  let other_half: (u16, BlockPosition) = if is_upper {
    block_properties.push(("half".to_string(), "lower".to_string()));
    let other_half_id = data::blocks::get_block_state_id_from_raw(block_states, &block_name, &block_properties);
    let other_half_location = BlockPosition { y: location.y - 1, ..location};
    (other_half_id, other_half_location)
  } else {
    block_properties.push(("half".to_string(), "upper".to_string()));
    let other_half_id = data::blocks::get_block_state_id_from_raw(block_states, &block_name, &block_properties);
    let other_half_location = BlockPosition { y: location.y + 1, ..location};
    (other_half_id, other_half_location)
  };

  return BlockInteractionResult::OverwriteBlocks(vec![(new_block_id, location), other_half]);
}

pub fn get_block_state_id(_face: u8, cardinal_direction: CardinalDirection, dimension: &Dimension, position: BlockPosition, used_item_name: &str, cursor_position_x: f32, _cursor_position_y: f32, cursor_position_z: f32, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  let facing = match cardinal_direction {
    CardinalDirection::North => DoorFacing::North,
    CardinalDirection::East => DoorFacing::East,
    CardinalDirection::South => DoorFacing::South,
    CardinalDirection::West => DoorFacing::West,
  };

  let open = DoorOpen::False;
  let powered = DoorPowered::False;

  let hinge_side = match cardinal_direction {
    CardinalDirection::North => if cursor_position_x > 0.5 { DoorHinge::Right } else { DoorHinge::Left },
    CardinalDirection::East => if cursor_position_z > 0.5 { DoorHinge::Right } else { DoorHinge::Left },
    CardinalDirection::South => if cursor_position_x < 0.5 { DoorHinge::Right } else { DoorHinge::Left },
    CardinalDirection::West => if cursor_position_z < 0.5 { DoorHinge::Right } else { DoorHinge::Left },
  };

  let position_to_check = match cardinal_direction {
    CardinalDirection::North => if hinge_side == DoorHinge::Left { BlockPosition { x: position.x - 1, ..position } } else { BlockPosition { x: position.x + 1, ..position } },
    CardinalDirection::East => if hinge_side == DoorHinge::Left { BlockPosition { z: position.z - 1, ..position } } else { BlockPosition { z: position.z + 1, ..position } },
    CardinalDirection::South => if hinge_side == DoorHinge::Left { BlockPosition { x: position.x + 1, ..position } } else { BlockPosition { x: position.x - 1, ..position } },
    CardinalDirection::West => if hinge_side == DoorHinge::Left { BlockPosition { z: position.z + 1, ..position } } else { BlockPosition { z: position.z - 1, ..position } },
  };
  let block_id_next_to_door = dimension.get_block(position_to_check).unwrap_or(0);
  let block_id_of_valid_door = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id;
  let we_must_go_double_door_mode = block_id_next_to_door == block_id_of_valid_door;
  let hinge = if we_must_go_double_door_mode { if hinge_side == DoorHinge::Right { DoorHinge::Left } else { DoorHinge::Right } } else { hinge_side };

  output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(hinge.clone())) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id, position));
  output.push((block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(facing.clone())) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(hinge.clone())) && x.properties.contains(&Property::DoorOpen(open.clone())) && x.properties.contains(&Property::DoorPowered(powered.clone()))).unwrap().id, BlockPosition { x: position.x, y: position.y + 1, z: position.z }));

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  mod interact {
    use super::*;

    #[test]
    fn open_door() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);
      let block_state_id_lower_closed = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      let block_state_id_lower_opened = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::True)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper_opened = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::True)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_lower_closed, 0, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_lower_opened, BlockPosition { x: 100, y: 80, z: -100 }),
        (block_state_id_upper_opened, BlockPosition { x: 100, y: 81, z: -100 })
      ]);

      assert_eq!(res, expected);
    }

    #[test]
    fn close_door() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);
      let block_state_id_lower_opened = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::True)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      let block_state_id_lower_closed = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper_closed = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::East)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      let res = interact(BlockPosition { x: 100, y: 80, z: -100 }, block_state_id_lower_opened, 0, &block_states);

      let expected = BlockInteractionResult::OverwriteBlocks(vec![
        (block_state_id_lower_closed, BlockPosition { x: 100, y: 80, z: -100 }),
        (block_state_id_upper_closed, BlockPosition { x: 100, y: 81, z: -100 })
      ]);

      assert_eq!(res, expected);
    }
  }

  mod get_block_state_id {
    use super::*;

    #[test]
    fn hinge_left_north() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);

      let block_state_id_lower = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;


      let res = get_block_state_id(0, CardinalDirection::North, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_door", 0.0, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id_lower, BlockPosition { x: 10, y: 80, z: 0 }),
        (block_state_id_upper, BlockPosition { x: 10, y: 81, z: 0 })
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn hinge_left_west() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);

      let block_state_id_lower = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::West)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::West)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;


      let res = get_block_state_id(0, CardinalDirection::West, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_door", 0.0, 0.0, 0.9, &block_states);

      let expected = vec![
        (block_state_id_lower, BlockPosition { x: 10, y: 80, z: 0 }),
        (block_state_id_upper, BlockPosition { x: 10, y: 81, z: 0 })
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn hinge_right_north() {
      let block_states = data::blocks::get_blocks();
      let dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);

      let block_state_id_lower = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Right)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Right)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;


      let res = get_block_state_id(0, CardinalDirection::North, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_door", 0.9, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id_lower, BlockPosition { x: 10, y: 80, z: 0 }),
        (block_state_id_upper, BlockPosition { x: 10, y: 81, z: 0 })
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn double_door() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);

      let block_state_id_lower = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_lower_double = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Right)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper_double = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Right)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_lower, &block_states).unwrap();
      dimension.overwrite_block(BlockPosition { x: 9, y: 81, z: 0 }, block_state_id_upper, &block_states).unwrap();

      let res = get_block_state_id(0, CardinalDirection::North, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_door", 0.0, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id_lower_double, BlockPosition { x: 10, y: 80, z: 0 }),
        (block_state_id_upper_double, BlockPosition { x: 10, y: 81, z: 0 })
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn double_door_z_offset_one() {
      let block_states = data::blocks::get_blocks();
      let mut dimension = Dimension::new();
      let block = data::blocks::get_block_from_name("minecraft:oak_door", &block_states);

      let block_state_id_lower = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Lower)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;
      let block_state_id_upper = block.states.iter().find(|x| x.properties.contains(&Property::DoorFacing(DoorFacing::North)) && x.properties.contains(&Property::DoorHalf(DoorHalf::Upper)) && x.properties.contains(&Property::DoorHinge(DoorHinge::Left)) && x.properties.contains(&Property::DoorOpen(DoorOpen::False)) && x.properties.contains(&Property::DoorPowered(DoorPowered::False))).unwrap().id;

      dimension.overwrite_block(BlockPosition { x: 9, y: 81, z: 0 }, block_state_id_lower, &block_states).unwrap();
      dimension.overwrite_block(BlockPosition { x: 9, y: 82, z: 0 }, block_state_id_upper, &block_states).unwrap();

      let res = get_block_state_id(0, CardinalDirection::North, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_door", 0.0, 0.0, 0.0, &block_states);

      let expected = vec![
        (block_state_id_lower, BlockPosition { x: 10, y: 80, z: 0 }),
        (block_state_id_upper, BlockPosition { x: 10, y: 81, z: 0 })
      ];

      assert_eq!(res, expected);
    }
  }
}
