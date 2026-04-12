use super::*;

pub fn tick(current_block_state_id: u16, block_states: &HashMap<String, Block>) -> u16 {
	let properties = data::blocks::get_block_state_from_block_state_id(current_block_state_id, block_states);
	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);
	if properties.properties.contains(&Property::BeetrootAge(BeetrootAge::Num0)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num1))).unwrap().id;
	} else if properties.properties.contains(&Property::BeetrootAge(BeetrootAge::Num1)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num2))).unwrap().id;
	} else if properties.properties.contains(&Property::BeetrootAge(BeetrootAge::Num2)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::BeetrootAge(BeetrootAge::Num3))).unwrap().id;
	} else {
		return current_block_state_id;
	}
}
