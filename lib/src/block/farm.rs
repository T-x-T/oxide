use super::*;

pub fn tick(
	current_block_state_id: u16,
	dimension: &Dimension,
	block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> u16 {
	let state = data::blocks::get_block_state_from_block_state_id(current_block_state_id, block_states);

	let mut found_water = false;
	'outer: for x in (block_position.x - 4)..=(block_position.x + 4) {
		for y in (block_position.y)..=(block_position.y + 1) {
			for z in (block_position.z - 4)..=(block_position.z + 4) {
				let block_to_check = dimension.get_block(BlockPosition {
					x,
					y,
					z,
				});

				if block_to_check.is_ok_and(|id| block_states.get("minecraft:water").unwrap().states.iter().any(|x| x.id == id)) {
					found_water = true;
					break 'outer;
				}
			}
		}
	}

	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);
	if found_water {
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num7)) {
			//is max moist
			return current_block_state_id;
		} else {
			//is *not* max moist, so make it
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num7))).unwrap().id;
		}
	} else {
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num7)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num6))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num6)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num5))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num5)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num4))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num4)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num3))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num3)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num2))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num2)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num1))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num1)) {
			return block.states.iter().find(|x| x.properties.contains(&Property::FarmMoisture(FarmMoisture::Num0))).unwrap().id;
		}
		if state.properties.contains(&Property::FarmMoisture(FarmMoisture::Num0)) {
			let dirt_block = block_states.get("minecraft:dirt").unwrap();
			return dirt_block.states[dirt_block.default_state].id;
		}
	}

	return current_block_state_id;
}
