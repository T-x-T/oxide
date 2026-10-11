use super::*;

pub fn generate(block_states: &HashMap<String, Block>) -> Vec<(BlockPosition, u16)> {
	let log = data::blocks::get_block_from_name("minecraft:acacia_log", block_states);
	let log_id = log.states[log.default_state].id;
	let leaves = data::blocks::get_block_from_name("minecraft:acacia_leaves", block_states);
	let leaves_id = leaves.states[leaves.default_state].id;

	return super::gen_tree(5, log_id, leaves_id);
}
