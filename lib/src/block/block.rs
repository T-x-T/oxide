use super::*;

pub fn get_hardness(_block_id: u16, block: Block, _block_states: &HashMap<String, Block>) -> f32 {
	if block.block_name.ends_with("_concrete") {
		return 1.8;
	} else if block.block_name.ends_with("terrcotta") {
		return 1.25;
	} else if block.block_name.ends_with("wool") {
		return 0.8;
	}

	match block.block_name {
		"minecraft:stone" => 1.5,
		"minecraft:andesite" => 1.5,
		"minecraft:ancient_debris" => 30.0,
		"minecraft:bedrock" => -1.0,
		"minecraft:blackstone" => 1.5,
		"minecraft:bookshelf" => 1.5,
		"minecraft:calcite" => 0.75,
		"minecraft:chiseled_deepslate" => 3.5,
		"minecraft:chiseled_polished_blackstone" => 1.5,
		"minecraft:chiseled_blackstone" => 3.5,
		"minecraft:chiseled_quartz_block" => 0.8,
		"minecraft:chiseled_red_sandstone" => 0.8,
		"minecraft:chiseled_resin_bricks" => 1.5,
		"minecraft:chiseled_sandstone" => 1.5,
		"minecraft:chiseled_stone_bricks" => 1.5,
		"minecraft:chiseled_tuff" => 1.5,
		"minecraft:chiseled_tuff_bricks" => 1.5,
		"minecraft:clay" => 0.6,
		"minecraft:coal_block" => 5.0,
		"minecraft:coarse_dirt" => 0.5,
		"minecraft:cobbled_deepslate" => 3.5,
		"minecraft:cracked_deepslate_bricks" => 3.5,
		"minecraft:cracked_deepslate_tiles" => 3.5,
		"minecraft:cracked_polished_blackstone_bricks" => 3.5,
		"minecraft:cracked_stone_bricks" => 1.5,
		"minecraft:cut_red_sandstone" => 0.8,
		"minecraft:cut_sandstone" => 0.8,
		"minecraft:dark_prismarine" => 1.5,
		"minecraft:dead_brain_coral_block" => 1.5,
		_ => 2.0,
	}
}
