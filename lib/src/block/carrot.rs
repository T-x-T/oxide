use super::*;

pub fn tick(
	current_block_state_id: u16,
	_dimension: &Dimension,
	_block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> u16 {
	let properties = data::blocks::get_block_state_from_block_state_id(current_block_state_id, block_states);
	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);
	if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num0)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num1))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num1)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num2))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num2)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num3))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num3)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num4))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num4)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num5))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num5)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num6))).unwrap().id;
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num6)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num7))).unwrap().id;
	} else {
		return current_block_state_id;
	}
}
