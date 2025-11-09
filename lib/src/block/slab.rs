use super::*;

pub fn get_block_state_id(face: u8, cursor_position_y: f32, dimension: &Dimension, position: BlockPosition, used_item_name: &str, block_states: &HashMap<String, Block>) -> Vec<(u16, BlockPosition)> {
  let block = data::blocks::get_block_from_name(used_item_name, block_states);
  let mut output: Vec<(u16, BlockPosition)> = Vec::new();

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

  return output;
}

#[cfg(test)]
mod test {
  use super::*;

  mod get_block_state_id {
    use super::*;

    #[test]
    fn on_top_of_other_block() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(1, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_of_other_block() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Top)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(0, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_upper_side_of_side_of_other_block() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Top)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(2, 0.51, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_side_of_side_of_other_block() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let dimension = Dimension::new();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(2, 0.49, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_top_of_bottom_slab() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Double)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(1, 0.0, &dimension, BlockPosition { x: 10, y: 81, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_of_bottom_slab() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 81, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Top)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(0, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_top_side_of_side_of_other_block_doubling_up() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Double)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(2, 0.49, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_side_of_side_of_other_block_doubling_up() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Top)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 80, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Double)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(2, 0.99, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_bottom_of_double_slab() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Double)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 81, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Top)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(0, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }

    #[test]
    fn on_top_of_double_slab() {
      let block_states = data::blocks::get_blocks();
      let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
      let mut dimension = Dimension::new();

      let block_state_id_to_place = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Double)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;
      dimension.overwrite_block(BlockPosition { x: 10, y: 79, z: 0 }, block_state_id_to_place, &block_states).unwrap();

      let block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom)) && x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))).unwrap().id;

      let res = get_block_state_id(1, 0.0, &dimension, BlockPosition { x: 10, y: 80, z: 0 }, "minecraft:oak_slab", &block_states);

      let expected = vec![
        (block_state_id, BlockPosition { x: 10, y: 80, z: 0 }),
      ];

      assert_eq!(res, expected);
    }
  }
}
