use crate::Position;
use data::blocks::Type;

pub fn interacted_with_block_at(location: Position, block_id_at_location: u16, face: u8) -> Vec<(u16, Position)> {
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

      vec![(new_block_id, location), other_half]
    },
    Type::Trapdoor => {
      let mut block_properties = data::blocks::get_raw_properties_from_block_state_id(&block_states, block_id_at_location);
      let is_open = block_properties.iter().find(|x| x.0 == "open").unwrap().1 == "true";
      block_properties.retain(|x| x.0 != "open");
      block_properties.push(("open".to_string(), if is_open { "false".to_string() } else { "true".to_string() }));

      let block_name = data::blocks::get_block_name_from_block_state_id(block_id_at_location, &block_states);
      let new_block_id = data::blocks::get_block_state_id_from_raw(&block_states, &block_name, block_properties.clone());

      vec![(new_block_id, location)]
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

      vec![(new_block_id, location)]
    },
    _ => Vec::new(),
  };
}
