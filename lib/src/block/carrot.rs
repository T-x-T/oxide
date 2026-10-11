use super::*;

pub fn tick(
	current_block_state_id: u16,
	_dimension: &Dimension,
	_block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> Vec<(BlockPosition, u16)> {
	let properties = data::blocks::get_block_state_from_block_state_id(current_block_state_id);
	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);
	let new_block_state_id = if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num0)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num1))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num1)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num2))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num2)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num3))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num3)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num4))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num4)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num5))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num5)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num6))).unwrap().id
	} else if properties.properties.contains(&Property::CarrotAge(CarrotAge::Num6)) {
		block.states.iter().find(|x| x.properties.contains(&Property::CarrotAge(CarrotAge::Num7))).unwrap().id
	} else {
		current_block_state_id
	};

	return vec![(BlockPosition::default(), new_block_state_id)];
}

pub fn update(position: BlockPosition, dimension: &Dimension, _block_states: &HashMap<String, Block>, block_id: u16) -> BlockUpdateOutcome {
	let position_to_check = BlockPosition { y: position.y - 1, ..position };
	if let Ok(block_id_to_check) = dimension.get_block(position_to_check) {
		//destroy self if were no longer on farmland
		if data::blocks::get_block_name_from_block_state_id(block_id_to_check) != "minecraft:farmland" {
			return BlockUpdateOutcome::DestroyAndDropSelf(block_id);
		}
	}

	return BlockUpdateOutcome::DoNothing;
}
