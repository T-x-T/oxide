use super::*;

pub fn tick(
	current_block_state_id: u16,
	_dimension: &Dimension,
	_block_position: BlockPosition,
	block_states: &HashMap<String, Block>,
) -> Vec<(BlockPosition, u16)> {
	let state = data::blocks::get_block_state_from_block_state_id(current_block_state_id);
	let block = data::blocks::get_block_from_block_state_id(current_block_state_id, block_states);

	if state.properties.contains(&Property::SaplingStage(SaplingStage::Num0)) {
		return vec![(
			BlockPosition::default(),
			block.states.iter().find(|x| x.properties.contains(&Property::SaplingStage(SaplingStage::Num1))).unwrap().id,
		)];
	} else if state.properties.contains(&Property::SaplingStage(SaplingStage::Num1)) {
		return match block.block_name {
			"minecraft:acacia_sapling" => crate::structures::trees::acacia::generate(block_states),
			"minecraft:birch_sapling" => crate::structures::trees::birch::generate(block_states),
			"minecraft:dark_oak_sapling" => crate::structures::trees::dark_oak::generate(block_states),
			"minecraft:jungle_sapling" => crate::structures::trees::jungle::generate(block_states),
			"minecraft:oak_sapling" => crate::structures::trees::oak::generate(block_states),
			"minecraft:spruce_sapling" => crate::structures::trees::spruce::generate(block_states),
			_ => crate::structures::trees::oak::generate(block_states),
		};
	}

	return vec![(BlockPosition::default(), current_block_state_id)];
}
