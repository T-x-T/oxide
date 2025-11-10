use super::*;

pub fn get_block_state_id(face: u8, cardinal_direction: CardinalDirection, cursor_position_y: f32, dimension: &Dimension, position: BlockPosition, used_item_name: &str, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

  let facing = match cardinal_direction {
    CardinalDirection::North => StairFacing::North,
    CardinalDirection::East => StairFacing::East,
    CardinalDirection::South => StairFacing::South,
    CardinalDirection::West => StairFacing::West,
  };

  let flip_it = face == 0 || (cursor_position_y > 0.5 && cursor_position_y < 0.9999);
  let stair_half = if flip_it { StairHalf::Top } else { StairHalf::Bottom };

  let north_block = dimension.get_block(BlockPosition { z: position.z - 1, ..position }).unwrap_or(0);
  let east_block = dimension.get_block(BlockPosition { x: position.x + 1, ..position }).unwrap_or(0);
  let south_block = dimension.get_block(BlockPosition { z: position.z + 1, ..position }).unwrap_or(0);
  let west_block = dimension.get_block(BlockPosition { x: position.x - 1, ..position }).unwrap_or(0);

  let stair_block_ids: Vec<u16> = block_states.iter()
    .filter(|x| x.1.block_type == Type::Stair)
    .flat_map(|x| {
      x.1.states.iter()
        .filter(|y| y.properties.contains(&Property::StairHalf(stair_half.clone())))
        .map(|y| y.id)
        .collect::<Vec<u16>>()
    })
    .collect();

  let mut shape = StairShape::Straight;

  if stair_block_ids.contains(&north_block) {
    let north_block_state = data::blocks::get_block_state_from_block_state_id(north_block, block_states);

    if north_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) && cardinal_direction != CardinalDirection::West || ( north_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || north_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerRight;
    } else if north_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) && cardinal_direction != CardinalDirection::East || ( north_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || north_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerLeft;
    }
  }

  if stair_block_ids.contains(&east_block) {
    let east_block_state = data::blocks::get_block_state_from_block_state_id(east_block, block_states);
    if east_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) && cardinal_direction != CardinalDirection::North || ( east_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || east_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerRight;
    } else if east_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) && cardinal_direction != CardinalDirection::South || ( east_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || east_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerLeft;
    }
  }

  if stair_block_ids.contains(&south_block) {
    let south_block_state = data::blocks::get_block_state_from_block_state_id(south_block, block_states);
    if south_block_state.properties.contains(&Property::StairFacing(StairFacing::East)) && cardinal_direction != CardinalDirection::East || ( south_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || south_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerRight;
    } else if south_block_state.properties.contains(&Property::StairFacing(StairFacing::West)) && cardinal_direction != CardinalDirection::West || ( south_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || south_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerLeft;
    }
  }

  if stair_block_ids.contains(&west_block) {
    let west_block_state = data::blocks::get_block_state_from_block_state_id(west_block, block_states);
    if west_block_state.properties.contains(&Property::StairFacing(StairFacing::South)) && cardinal_direction != CardinalDirection::South || ( west_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || west_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerRight;
    } else if west_block_state.properties.contains(&Property::StairFacing(StairFacing::North)) && cardinal_direction != CardinalDirection::North || ( west_block_state.properties.contains(&Property::StairShape(StairShape::OuterLeft)) || west_block_state.properties.contains(&Property::StairShape(StairShape::InnerRight)) ) {
      shape = StairShape::InnerLeft;
    }
  }

  output.push((block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(facing.clone())) && x.properties.contains(&Property::StairHalf(stair_half.clone())) && x.properties.contains(&Property::StairShape(shape.clone())) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id, position));

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  mod get_block_state_id {
    use super::*;

    #[test]
    fn on_top_of_other_block_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_top_of_other_block_east() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(1, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_of_other_block_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Top)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(0, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_of_other_block_east() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Top)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(0, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_lower_side_of_other_block_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(3, CardinalDirection::North, 0.49, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_right_north_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerRight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_left_north_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerLeft)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_right_east_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerRight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_left_east_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerLeft)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_right_south_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::South, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerRight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_left_south_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::South, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerLeft)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_right_west_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::West, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerRight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn curved_stair_inner_left_west_bottom() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::West, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::InnerLeft)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_surrounded_on_all_sides_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place, &block_states).unwrap();
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_west_straight_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_east_straight_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::North, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_north_straight_east() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_south_straight_east() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::East, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::East)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_east_straight_south() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 11, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::South, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_west_straight_south() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 9, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::South, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::South)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_south_straight_west() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::West, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn straight_stair_north_straight_west() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: -1 }, block_state_id_to_place, &block_states).unwrap();

      let res = get_block_state_id(1, CardinalDirection::West, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::West)) && x.properties.contains(&Property::StairHalf(StairHalf::Bottom)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;
      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }
  }
}

//TODO: dont forget to add tests for different stair blocks combining together and different halves not interacting with each other, also test what happens with multiple stairs around
