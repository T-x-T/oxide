use super::*;

pub fn tick(
	current_block_state_id: u16,
	_dimension: &Dimension,
	_block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> u16 {
	let properties = data::blocks::get_block_state_from_block_state_id(current_block_state_id, block_states);
	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);
	if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num0)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num1))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num1)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num2))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num2)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num3))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num3)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num4))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num4)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num5))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num5)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num6))).unwrap().id;
	} else if properties.properties.contains(&Property::PotatoAge(PotatoAge::Num6)) {
		return block.states.iter().find(|x| x.properties.contains(&Property::PotatoAge(PotatoAge::Num7))).unwrap().id;
	} else {
		return current_block_state_id;
	}
}

pub fn update(position: BlockPosition, dimension: &Dimension, block_states: &HashMap<String, Block>) -> Option<u16> {
	if let Ok(block_id_to_check) = dimension.get_block(position) {
		//destroy self if were no longer on farmland
		if data::blocks::get_block_name_from_block_state_id(block_id_to_check, block_states) != "minecraft:farmland" {
			return Some(0);
		}
	}

	return None;
}
