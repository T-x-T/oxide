use super::*;

pub fn get_block_state_id(
	face: u8,
	cursor_position_y: f32,
	dimension: &Dimension,
	position: BlockPosition,
	used_item_name: &str,
	block_states: &HashMap<String, Block>,
) -> Vec<(u16, BlockPosition)> {
	let block = data::blocks::get_block_from_name(used_item_name, block_states);
	let mut output: Vec<(u16, BlockPosition)> = Vec::new();

	let position_to_check = if face == 0 {
		BlockPosition {
			y: position.y + 1,
			..position
		}
	} else if face == 1 {
		BlockPosition {
			y: position.y - 1,
			..position
		}
	} else {
		position
	};
	let block_id_at_position_to_check = dimension.get_block(position_to_check).unwrap_or(0);
	let block_id_at_position = dimension.get_block(position).unwrap_or(0);

	let block_ids_of_half_slabs: Vec<u16> =
		block.states.iter().filter(|x| !x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();
	let block_ids_of_top_slabs: Vec<u16> =
		block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Top))).map(|x| x.id).collect();
	let block_ids_of_bottom_slabs: Vec<u16> =
		block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Bottom))).map(|x| x.id).collect();
	let block_ids_of_double_slabs: Vec<u16> =
		block.states.iter().filter(|x| x.properties.contains(&Property::SlabType(SlabType::Double))).map(|x| x.id).collect();

	let placed_underneath_bottom_slab = block_ids_of_bottom_slabs.contains(&block_id_at_position_to_check) && face == 0;
	let double_up_placed_underneath_bottom_slab = placed_underneath_bottom_slab && block_ids_of_bottom_slabs.contains(&block_id_at_position);
	let placed_ontop_top_slab = block_ids_of_top_slabs.contains(&block_id_at_position_to_check) && face == 1;
	let double_up_placed_ontop_top_slab = placed_ontop_top_slab && block_ids_of_top_slabs.contains(&block_id_at_position);
	let double_it_up =
		(block_ids_of_half_slabs.contains(&block_id_at_position_to_check) && !placed_underneath_bottom_slab && !placed_ontop_top_slab)
			|| double_up_placed_underneath_bottom_slab
			|| double_up_placed_ontop_top_slab;

	let place_top = (face >= 2 && cursor_position_y > 0.5) || face == 0;
	let slab_type_to_place = if double_it_up {
		SlabType::Double
	} else if place_top {
		SlabType::Top
	} else {
		SlabType::Bottom
	};

	let its_already_doubled = block_ids_of_double_slabs.contains(&block_id_at_position_to_check);
	let position_to_place = if its_already_doubled {
		position
	} else if double_it_up {
		position_to_check
	} else {
		position
	};

	output.push((
		block
			.states
			.iter()
			.find(|x| {
				x.properties.contains(&Property::SlabType(slab_type_to_place.clone()))
					&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
			})
			.unwrap()
			.id,
		position_to_place,
	));

	return output;
}

pub fn get_item_drop(
	block: data::blocks::Block,
	used_tool: &data::items::Item,
	_block_states: &HashMap<String, data::blocks::Block>,
) -> Item {
	let all_items = data::items::get_items();
	let pickaxes: Vec<i32> = data::tags::get_item().get("pickaxes").unwrap().iter().map(|x| all_items.get(x).unwrap().id).collect();
	let stone_pickaxes = [
		all_items.get("minecraft:stone_pickaxe").unwrap().id,
		all_items.get("minecraft:copper_pickaxe").unwrap().id,
		all_items.get("minecraft:iron_pickaxe").unwrap().id,
		all_items.get("minecraft:golden_pickaxe").unwrap().id,
		all_items.get("minecraft:diamond_pickaxe").unwrap().id,
		all_items.get("minecraft:netherite_pickaxe").unwrap().id,
	];

	let needs_to_be_mined_with_wooden_pickaxe = [
		"minecraft:andesite_slab",
		"minecraft:blackstone_slab",
		"minecraft:brick_slab",
		"minecraft:cobbled_deepslate_slab",
		"minecraft:cobblestone_slab",
		"minecraft:cut_red_sandstone_slab",
		"minecraft:cut_sandstone_slab",
		"minecraft:dark_prismarine_slab",
		"minecraft:deepslate_brick_slab",
		"minecraft:deepslate_tile_slab",
		"minecraft:diorite_slab",
		"minecraft:end_stone_brick_slab",
		"minecraft:granite_slab",
		"minecraft:mossy_cobblestone_slab",
		"minecraft:mossy_stone_brick_slab",
		"minecraft:mud_brick_slab",
		"minecraft:nether_brick_slab",
		"minecraft:polished_andesite_slab",
		"minecraft:polished_blackstone_brick_slab",
		"minecraft:polished_blackstone_slab",
		"minecraft:polished_deepslate_slab",
		"minecraft:polished_diorite_slab",
		"minecraft:polished_granite_slab",
		"minecraft:polished_tuff_slab",
		"minecraft:prismarine_brick_slab",
		"minecraft:prismarine_slab",
		"minecraft:purpur_slab",
		"minecraft:quartz_slab",
		"minecraft:red_nether_brick_slab",
		"minecraft:red_sandstone_slab",
		"minecraft:resin_brick_slab",
		"minecraft:sandstone_slab",
		"minecraft:smooth_quartz_slab",
		"minecraft:smooth_red_sandstone_slab",
		"minecraft:smooth_sandstone_slab",
		"minecraft:smooth_stone_slab",
		"minecraft:stone_brick_slab",
		"minecraft:stone_slab",
		"minecraft:tuff_brick_slab",
		"minecraft:tuff_slab",
	];
	let needs_to_be_mined_with_stone_pickaxe = [
		"minecraft:waxed_cut_copper_slab",
		"minecraft:waxed_exposed_cut_copper_slab",
		"minecraft:waxed_oxidized_cut_copper_slab",
		"minecraft:waxed_weathered_cut_copper_slab",
	];
	if needs_to_be_mined_with_wooden_pickaxe.contains(&block.block_name) && !pickaxes.contains(&used_tool.id) {
		return Item::default();
	}

	if needs_to_be_mined_with_stone_pickaxe.contains(&block.block_name) && !stone_pickaxes.contains(&used_tool.id) {
		return Item::default();
	}

	return Item {
		id: block.block_name.to_string(),
		count: 1,
		components: Vec::new(),
	};
}

pub fn get_hardness(_block_id: u16, block: data::blocks::Block, _block_states: &HashMap<String, data::blocks::Block>) -> f32 {
	match block.block_name {
		"minecraft:andesite_slab" => 1.5,
		"minecraft:cobbled_deepslate_slab" => 3.5,
		"minecraft:dark_prismarine_slab" => 1.5,
		"minecraft:deepslate_brick_slab" => 3.5,
		"minecraft:deepslate_tile_slab" => 3.5,
		"minecraft:diorite_slab" => 1.5,
		"minecraft:end_stone_brick_slab" => 3.0,
		"minecraft:granite_slab" => 1.5,
		"minecraft:mossy_stone_brick_slab" => 1.5,
		"minecraft:mud_brick_slab" => 1.5,
		"minecraft:polished_andesite_slab" => 1.5,
		"minecraft:polished_deepslate_slab" => 3.5,
		"minecraft:polished_diorite_slab" => 1.5,
		"minecraft:polished_granite_slab" => 1.5,
		"minecraft:polished_tuff_slab" => 1.5,
		"minecraft:prismarine_brick_slab" => 1.5,
		"minecraft:prismarine_slab" => 1.5,
		"minecraft:resin_brick_slab" => 1.5,
		"minecraft:tuff_brick_slab" => 1.5,
		"minecraft:tuff_slab" => 1.5,
		"minecraft:waxed_cut_copper_slab" => 3.0,
		"minecraft:waxed_exposed_cut_copper_slab" => 3.0,
		"minecraft:waxed_oxidized_cut_copper_slab" => 3.0,
		"minecraft:waxed_weathered_cut_copper_slab" => 3.0,
		_ => 2.0,
	}
}

#[cfg(test)]
mod test {
	use super::*;

	mod get_block_state_id {
		use super::*;

		#[test]
		fn on_top_of_other_block() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				1,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_bottom_of_other_block() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Top))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_upper_side_of_side_of_other_block() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Top))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				2,
				0.51,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_bottom_side_of_side_of_other_block() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let dimension = Dimension::new();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				2,
				0.49,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_top_of_bottom_slab() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Double))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				1,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 81,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_bottom_of_bottom_slab() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 81,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Top))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_top_side_of_side_of_other_block_doubling_up() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Double))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				2,
				0.49,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_bottom_side_of_side_of_other_block_doubling_up() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Top))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 80,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Double))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				2,
				0.99,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_bottom_of_double_slab() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Double))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 81,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Top))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				0,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}

		#[test]
		fn on_top_of_double_slab() {
			let block_states = data::blocks::get_blocks();
			let block = data::blocks::get_block_from_name("minecraft:oak_slab", &block_states);
			let mut dimension = Dimension::new();

			let block_state_id_to_place = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Double))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;
			dimension
				.overwrite_block(
					BlockPosition {
						x: 10,
						y: 79,
						z: 0,
					},
					block_state_id_to_place,
				)
				.unwrap();

			let block_state_id = block
				.states
				.iter()
				.find(|x| {
					x.properties.contains(&Property::SlabType(SlabType::Bottom))
						&& x.properties.contains(&Property::SlabWaterlogged(SlabWaterlogged::False))
				})
				.unwrap()
				.id;

			let res = get_block_state_id(
				1,
				0.0,
				&dimension,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
				"minecraft:oak_slab",
				&block_states,
			);

			let expected = vec![(
				block_state_id,
				BlockPosition {
					x: 10,
					y: 80,
					z: 0,
				},
			)];

			assert_eq!(res, expected);
		}
	}

	mod get_item_drop {
		#[test]
		fn copper_slab_drops_with_stone_pickaxe() {
			let block_states = data::blocks::get_blocks();
			let all_items = data::items::get_items();

			let res = super::get_item_drop(
				block_states.get("minecraft:waxed_weathered_cut_copper_slab").unwrap().clone(),
				all_items.get("minecraft:stone_pickaxe").unwrap(),
				&block_states,
			);

			assert_eq!(res.id, "minecraft:waxed_weathered_cut_copper_slab");
			assert_eq!(res.count, 1);
		}

		#[test]
		fn copper_slab_drops_nothing_with_wooden_pickaxe() {
			let block_states = data::blocks::get_blocks();
			let all_items = data::items::get_items();

			let res = super::get_item_drop(
				block_states.get("minecraft:waxed_weathered_cut_copper_slab").unwrap().clone(),
				all_items.get("minecraft:wooden_pickaxe").unwrap(),
				&block_states,
			);

			assert_eq!(res.id, "minecraft:air");
			assert_eq!(res.count, 0);
		}
	}
}
