use data::blocks::Property;

pub fn get_block_state_id(face: u8, used_item_name: String) -> i32 {
  let block = data::blocks::get_block_from_name(used_item_name);
  let mut block_state_id = block.states.iter().find(|x| x.default).unwrap().id;

  if block.properties.contains(&Property::RotatedPillarAxis(data::blocks::RotatedPillarAxis::X)) {
    if face == 4 || face == 5 {
      block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(data::blocks::RotatedPillarAxis::X))).unwrap().id;
    }
  }
  if block.properties.contains(&Property::RotatedPillarAxis(data::blocks::RotatedPillarAxis::Z)) {
    if face == 2 || face == 3 {
      block_state_id = block.states.iter().find(|x| x.properties.contains(&Property::RotatedPillarAxis(data::blocks::RotatedPillarAxis::Z))).unwrap().id;
    }
  }

  return block_state_id;
}
