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

  output.push((block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(facing.clone())) && x.properties.contains(&Property::StairHalf(stair_half.clone())) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id, position));

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
    fn on_upper_side_of_other_block_north() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_stairs", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::StairFacing(StairFacing::North)) && x.properties.contains(&Property::StairHalf(StairHalf::Top)) && x.properties.contains(&Property::StairShape(StairShape::Straight)) && x.properties.contains(&Property::StairWaterlogged(StairWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(3, CardinalDirection::North, 0.51, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_stairs", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }
  }
}
