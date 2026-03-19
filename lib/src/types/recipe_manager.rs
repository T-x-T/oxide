use super::*;
use basic_types::recipe::{CraftingShapedData, FurnaceData, Recipe};
use std::collections::HashMap;

pub struct RecipeManager {
	recipes: HashMap<&'static str, Recipe>,
}

impl RecipeManager {
	pub fn new(recipes: HashMap<&'static str, Recipe>) -> RecipeManager {
		return RecipeManager {
			recipes,
		};
	}

	pub fn get_crafting_recipe_2x2(&self, slots: &[Option<Slot>; 4]) -> Option<&Recipe> {
		if slots.iter().all(|x| x.is_none()) {
			return None;
		}
		let matching_recipes: Vec<(&&'static str, &Recipe)> =
			self.recipes.iter().filter(|(_, recipe)| is_recipe_a_match_2x2(slots, recipe)).collect();

		if matching_recipes.len() > 1 {
			println!("found multiple matching recipes for slots {slots:?}:\n{matching_recipes:?}");
		}

		if matching_recipes.is_empty() {
			return None;
		} else {
			return Some(matching_recipes.first().unwrap().1);
		}
	}

	pub fn get_crafting_recipe_3x3(&self, slots: &[Option<Slot>; 9]) -> Option<&Recipe> {
		if slots.iter().all(|x| x.is_none()) {
			return None;
		}
		let matching_recipes: Vec<(&&'static str, &Recipe)> =
			self.recipes.iter().filter(|(_, recipe)| is_recipe_a_match_3x3(slots, recipe)).collect();
		if matching_recipes.len() > 1 {
			println!("found multiple matching recipes for slots {slots:?}:\n{matching_recipes:?}");
		}

		if matching_recipes.is_empty() {
			return None;
		} else {
			return Some(matching_recipes.first().unwrap().1);
		}
	}

	pub fn get_smelting_recipe(&self, slot: &Slot) -> Option<FurnaceData> {
		for recipe in self.recipes.values() {
			match recipe {
				Recipe::Smelting(recipe) => {
					let mut tag_resolved_ingredient_options: Vec<&str> = Vec::new();
					for ingredient_option in &recipe.ingredient {
						let mut tag_resolved_ingredient_option: Vec<&str> = if ingredient_option.starts_with("#") {
							data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
						} else {
							vec![ingredient_option]
						};
						tag_resolved_ingredient_options.append(&mut tag_resolved_ingredient_option);
					}

					if tag_resolved_ingredient_options.contains(&data::items::get_item_name_by_id(slot.id)) {
						return Some(*recipe.clone());
					}
				}
				_ => continue,
			}
		}

		return None;
	}

	pub fn get_smoking_recipe(&self, slot: &Slot) -> Option<FurnaceData> {
		for recipe in self.recipes.values() {
			match recipe {
				Recipe::Smoking(recipe) => {
					let mut tag_resolved_ingredient_options: Vec<&str> = Vec::new();
					for ingredient_option in &recipe.ingredient {
						let mut tag_resolved_ingredient_option: Vec<&str> = if ingredient_option.starts_with("#") {
							data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
						} else {
							vec![ingredient_option]
						};
						tag_resolved_ingredient_options.append(&mut tag_resolved_ingredient_option);
					}

					if tag_resolved_ingredient_options.contains(&data::items::get_item_name_by_id(slot.id)) {
						return Some(*recipe.clone());
					}
				}
				_ => continue,
			}
		}

		return None;
	}

	pub fn get_blasting_recipe(&self, slot: &Slot) -> Option<FurnaceData> {
		for recipe in self.recipes.values() {
			match recipe {
				Recipe::Blasting(recipe) => {
					let mut tag_resolved_ingredient_options: Vec<&str> = Vec::new();
					for ingredient_option in &recipe.ingredient {
						let mut tag_resolved_ingredient_option: Vec<&str> = if ingredient_option.starts_with("#") {
							data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
						} else {
							vec![ingredient_option]
						};
						tag_resolved_ingredient_options.append(&mut tag_resolved_ingredient_option);
					}

					if tag_resolved_ingredient_options.contains(&data::items::get_item_name_by_id(slot.id)) {
						return Some(*recipe.clone());
					}
				}
				_ => continue,
			}
		}

		return None;
	}

	//move to some better place?; data can be found here: https://minecraft.wiki/w/Smelting#Fuel
	pub fn get_fuel_burning_time(&self, item_id: &str) -> i32 {
		if item_id.ends_with("_boat") {
			return 1200;
		}
		if item_id.ends_with("_log")
			|| item_id.ends_with("_wood")
			|| item_id.ends_with("_planks")
			|| item_id.ends_with("_fence_gate")
			|| item_id.ends_with("_fence")
			|| item_id.ends_with("_shelf")
			|| item_id.ends_with("_banner")
		{
			return 300;
		}
		if item_id.ends_with("wool") {
			return 100;
		}
		if item_id.ends_with("carpet") {
			return 67;
		}
		return match item_id {
			"minecraft:lava_bucket" => 20_000,
			"minecraft:coal_block" => 16_000,
			"minecraft:dried_kelp_block" => 4000,
			"minecraft:blaze_rod" => 2400,
			"minecraft:coal" => 1600,
			"minecraft:charcoal" => 1600,
			"minecraft:bamboo_mosaic" => 300,
			"minecraft:bamboo_mosaic_slab" => 150,
			"minecraft:bamboo_mosaic_stairs" => 300,
			"minecraft:chiseled_bookshelf" => 300,
			"minecraft:bamboo_block" => 300,
			"minecraft:crimson_stem" => 300,
			"minecraft:warped_stem" => 300,
			"minecraft:stripped_crimson_stem" => 300,
			"minecraft:stripped_warped_stem" => 300,
			"minecraft:crimson_hyphae" => 300,
			"minecraft:warped_hyphae" => 300,
			"minecraft:stripped_crimson_hyphae" => 300,
			"minecraft:stripped_warped_hyphae" => 300,
			"minecraft:oak_slab" => 150,
			"minecraft:spruce_slab" => 150,
			"minecraft:birch_slab" => 150,
			"minecraft:jungle_slab" => 150,
			"minecraft:acacia_slab" => 150,
			"minecraft:dark_oak_slab" => 150,
			"minecraft:mangrove_slab" => 150,
			"minecraft:cherry_slab" => 150,
			"minecraft:pale_oak_slab" => 150,
			"minecraft:bamboo_slab" => 150,
			"minecraft:crimson_slab" => 150,
			"minecraft:warped_slab" => 150,
			"minecraft:oak_stairs" => 300,
			"minecraft:spruce_stairs" => 300,
			"minecraft:birch_stairs" => 300,
			"minecraft:jungle_stairs" => 300,
			"minecraft:acacia_stairs" => 300,
			"minecraft:dark_oak_stairs" => 300,
			"minecraft:mangrove_stairs" => 300,
			"minecraft:cherry_stairs" => 300,
			"minecraft:pale_oak_stairs" => 300,
			"minecraft:bamboo_stairs" => 300,
			"minecraft:crimson_stairs" => 300,
			"minecraft:warped_stairs" => 300,
			"minecraft:oak_pressure_plate" => 300,
			"minecraft:spruce_pressure_plate" => 300,
			"minecraft:birch_pressure_plate" => 300,
			"minecraft:jungle_pressure_plate" => 300,
			"minecraft:acacia_pressure_plate" => 300,
			"minecraft:dark_oak_pressure_plate" => 300,
			"minecraft:mangrove_pressure_plate" => 300,
			"minecraft:cherry_pressure_plate" => 300,
			"minecraft:pale_oak_pressure_plate" => 300,
			"minecraft:bamboo_pressure_plate" => 300,
			"minecraft:crimson_pressure_plate" => 300,
			"minecraft:warped_pressure_plate" => 300,
			"minecraft:oak_button" => 100,
			"minecraft:spruce_button" => 100,
			"minecraft:birch_button" => 100,
			"minecraft:jungle_button" => 100,
			"minecraft:acacia_button" => 100,
			"minecraft:dark_oak_button" => 100,
			"minecraft:mangrove_button" => 100,
			"minecraft:cherry_button" => 100,
			"minecraft:pale_oak_button" => 100,
			"minecraft:bamboo_button" => 100,
			"minecraft:crimson_button" => 100,
			"minecraft:warped_button" => 100,
			"minecraft:oak_trapdoor" => 300,
			"minecraft:spruce_trapdoor" => 300,
			"minecraft:birch_trapdoor" => 300,
			"minecraft:jungle_trapdoor" => 300,
			"minecraft:acacia_trapdoor" => 300,
			"minecraft:dark_oak_trapdoor" => 300,
			"minecraft:mangrove_trapdoor" => 300,
			"minecraft:cherry_trapdoor" => 300,
			"minecraft:pale_oak_trapdoor" => 300,
			"minecraft:bamboo_trapdoor" => 300,
			"minecraft:crimson_trapdoor" => 300,
			"minecraft:warped_trapdoor" => 300,
			"minecraft:oak_door" => 200,
			"minecraft:spruce_door" => 200,
			"minecraft:birch_door" => 200,
			"minecraft:jungle_door" => 200,
			"minecraft:acacia_door" => 200,
			"minecraft:dark_oak_door" => 200,
			"minecraft:mangrove_door" => 200,
			"minecraft:cherry_door" => 200,
			"minecraft:pale_oak_door" => 200,
			"minecraft:bamboo_door" => 200,
			"minecraft:crimson_door" => 200,
			"minecraft:warped_door" => 200,
			"minecraft:oak_sign" => 200,
			"minecraft:spruce_sign" => 200,
			"minecraft:birch_sign" => 200,
			"minecraft:jungle_sign" => 200,
			"minecraft:acacia_sign" => 200,
			"minecraft:dark_oak_sign" => 200,
			"minecraft:mangrove_sign" => 200,
			"minecraft:cherry_sign" => 200,
			"minecraft:pale_oak_sign" => 200,
			"minecraft:bamboo_sign" => 200,
			"minecraft:crimson_sign" => 200,
			"minecraft:warped_sign" => 200,
			"minecraft:oak_hanging_åsign" => 200,
			"minecraft:spruce_hanging_sign" => 200,
			"minecraft:birch_hanging_sign" => 200,
			"minecraft:jungle_hanging_sign" => 200,
			"minecraft:acacia_hanging_sign" => 200,
			"minecraft:dark_oak_hanging_sign" => 200,
			"minecraft:mangrove_hanging_sign" => 200,
			"minecraft:cherry_hanging_sign" => 200,
			"minecraft:pale_oak_hanging_sign" => 200,
			"minecraft:bamboo_hanging_sign" => 200,
			"minecraft:crimson_hanging_sign" => 200,
			"minecraft:warped_hanging_sign" => 200,
			"minecraft:mangrove_roots" => 300,
			"minecraft:ladder" => 300,
			"minecraft:crafting_table" => 300,
			"minecraft:cartography_table" => 300,
			"minecraft:fletching_table" => 300,
			"minecraft:smithing_table" => 300,
			"minecraft:loom" => 300,
			"minecraft:bookshelf" => 300,
			"minecraft:lectern" => 300,
			"minecraft:composter" => 300,
			"minecraft:chest" => 300,
			"minecraft:trapped_chest" => 300,
			"minecraft:barrel" => 300,
			"minecraft:daylight_detector" => 300,
			"minecraft:jukebox" => 300,
			"minecraft:note_block" => 300,
			"minecraft:crossbow" => 300,
			"minecraft:bow" => 300,
			"minecraft:fishing_rod" => 300,
			"minecraft:wooden_pickaxe" => 200,
			"minecraft:wooden_shovel" => 200,
			"minecraft:wooden_hoe" => 200,
			"minecraft:wooden_axe" => 200,
			"minecraft:wooden_sword" => 200,
			"minecraft:wooden_spear" => 200,
			"minecraft:bowl" => 100,
			"minecraft:oak_sapling" => 100,
			"minecraft:spruce_sapling" => 100,
			"minecraft:birch_sapling" => 100,
			"minecraft:jungle_sapling" => 100,
			"minecraft:acacia_sapling" => 100,
			"minecraft:dark_oak_sapling" => 100,
			"minecraft:pale_oak_sapling" => 100,
			"minecraft:azalea" => 100,
			"minecraft:flowering_azalea" => 100,
			"minecraft:mangrove_propagule" => 100,
			"minecraft:cherry_sapling" => 100,
			"minecraft:stick" => 100,
			"minecraft:dead_bush" => 100,
			"minecraft:azaela" => 100,
			"minecraft:leaf_litter" => 100,
			"minecraft:short_dry_grass" => 100,
			"minecraft:tall_dry_grass" => 100,
			"minecraft:bamboo" => 50,
			"minecraft:scaffolding" => 50,
			_ => 0,
		};
	}
}

fn is_recipe_a_match_2x2(slots: &[Option<Slot>; 4], recipe: &Recipe) -> bool {
	return match recipe {
		Recipe::CraftingShaped(crafting_shaped_data) => {
			if crafting_shaped_data.pattern.len() > 2 || crafting_shaped_data.pattern.first().unwrap().len() > 2 {
				return false;
			}

			let mut possible_shapes: Vec<[Option<Vec<&str>>; 4]> = Vec::new();

			if crafting_shaped_data.pattern.len() == 2 && crafting_shaped_data.pattern.first().unwrap().len() == 2 {
				// XX
				// XX
				let mut index = 0;
				let mut shape: [Option<Vec<&str>>; 4] = [None, None, None, None];
				for row in &crafting_shaped_data.pattern {
					for key in row.chars() {
						if key == ' ' {
							shape[index] = None;
						} else {
							let possible_options = crafting_shaped_data.key.get(key.to_string().as_str()).unwrap();
							let mut all_possible_options: Vec<&str> = Vec::new();
							for ingredient_option in possible_options {
								let mut tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
									data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
								} else {
									vec![ingredient_option]
								};
								all_possible_options.append(&mut tag_resolved_ingredient_options);
							}
							shape[index] = Some(all_possible_options);
						}
						index += 1;
					}
				}
				let reversed_shape: [Option<Vec<&str>>; 4] = [shape[1].clone(), shape[0].clone(), shape[3].clone(), shape[2].clone()];
				possible_shapes.push(reversed_shape);
				possible_shapes.push(shape);
			} else if crafting_shaped_data.pattern.len() == 2 && crafting_shaped_data.pattern.first().unwrap().len() == 1 {
				// X
				// X
				for variant in 0..=1 {
					let mut index = 0;
					let mut shape: [Option<Vec<&str>>; 4] = [None, None, None, None];
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							let possible_options = crafting_shaped_data.key.get(key.to_string().as_str()).unwrap();
							let mut all_possible_options: Vec<&str> = Vec::new();
							for ingredient_option in possible_options {
								let mut tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
									data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
								} else {
									vec![ingredient_option]
								};
								all_possible_options.append(&mut tag_resolved_ingredient_options);
							}
							shape[index + variant] = Some(all_possible_options);
							index += 2;
						}
					}
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 1 && crafting_shaped_data.pattern.first().unwrap().len() == 2 {
				// XX
				for variant in 0..=1 {
					let mut index = 0;
					let mut shape: [Option<Vec<&str>>; 4] = [None, None, None, None];
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							let possible_options = crafting_shaped_data.key.get(key.to_string().as_str()).unwrap();
							let mut all_possible_options: Vec<&str> = Vec::new();
							for ingredient_option in possible_options {
								let mut tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
									data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
								} else {
									vec![ingredient_option]
								};
								all_possible_options.append(&mut tag_resolved_ingredient_options);
							}
							shape[index + (variant * 2)] = Some(all_possible_options);
							index += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 4] = [shape[1].clone(), shape[0].clone(), shape[3].clone(), shape[2].clone()];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else {
				println!("is_recipe_a_match_2x2 encountered a shaped crafting recipe thats a weird shape:\n{crafting_shaped_data:?}");
			}

			let mut match_found = false;
			'outer: for possible_shape in possible_shapes {
				for (i, slot) in possible_shape.iter().enumerate() {
					if let Some(recipe_slot) = slot {
						if let Some(input_slot) = &slots[i] {
							if !recipe_slot.contains(&data::items::get_item_name_by_id(input_slot.id)) {
								continue 'outer;
							}
						} else {
							continue 'outer;
						}
					} else {
						if slots[i].as_ref().is_some_and(|x| x.count > 0) {
							continue 'outer;
						}
					}
				}
				match_found = true;
				break;
			}

			match_found
		}
		Recipe::CraftingShapeless(crafting_shapeless_data) => {
			let mut slots_vec = slots.to_vec();

			for ingredient in &crafting_shapeless_data.ingredients {
				let mut found_match = false;
				'ingredient_loop: for ingredient_option in ingredient {
					let tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
						data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
					} else {
						vec![ingredient_option]
					};
					for actual_ingredient_option in tag_resolved_ingredient_options {
						for slot in &mut slots_vec {
							if slot.as_ref().is_some_and(|x| data::items::get_item_name_by_id(x.id) == actual_ingredient_option) {
								*slot = None;
								found_match = true;
								break 'ingredient_loop;
							}
						}
					}
				}
				if !found_match {
					return false;
				}
			}

			slots_vec.iter().filter(|x| x.is_some()).collect::<Vec<&Option<Slot>>>().is_empty()
		}
		Recipe::CraftingTransmute(_crafting_transmute_data) => false,
		Recipe::CraftingSpecial(_crafting_special_data) => false,
		Recipe::CraftingDecoratedPot(_category) => false,
		_ => false,
	};
}

fn is_recipe_a_match_3x3(slots: &[Option<Slot>; 9], recipe: &Recipe) -> bool {
	return match recipe {
		Recipe::CraftingShaped(crafting_shaped_data) => {
			let used_item_count = slots.iter().filter(|x| x.is_some()).count();
			if crafting_shaped_data.pattern.len() > used_item_count || crafting_shaped_data.pattern.first().unwrap().len() > used_item_count {
				return false;
			}

			let mut possible_shapes: Vec<[Option<Vec<&str>>; 9]> = Vec::new();

			let calculated_key: HashMap<char, Vec<&str>> = crafting_shaped_data
				.key
				.keys()
				.map(|k| (k.chars().next().unwrap(), process_key(crafting_shaped_data, k.chars().next().unwrap()).unwrap()))
				.collect();

			if crafting_shaped_data.pattern.len() == 3 && crafting_shaped_data.pattern.first().unwrap().len() == 3 {
				// XXX
				// XXX
				// XXX
				let mut offset = 0;
				let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
				for row in &crafting_shaped_data.pattern {
					for key in row.chars() {
						shape[offset] = calculated_key.get(&key).cloned();

						offset += 1;
					}
				}
				let reversed_shape: [Option<Vec<&str>>; 9] = [
					shape[2].clone(),
					shape[1].clone(),
					shape[0].clone(),
					shape[5].clone(),
					shape[4].clone(),
					shape[3].clone(),
					shape[8].clone(),
					shape[7].clone(),
					shape[6].clone(),
				];
				possible_shapes.push(reversed_shape);
				possible_shapes.push(shape);
			} else if crafting_shaped_data.pattern.len() == 2 && crafting_shaped_data.pattern.first().unwrap().len() == 1 {
				// X
				// X
				for starting_index in 0..=5 {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							shape[starting_index + (offset * 3)] = calculated_key.get(&key).cloned();
							offset += 1;
						}
					}
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 1 && crafting_shaped_data.pattern.first().unwrap().len() == 2 {
				// XX
				for starting_index in [0, 1, 3, 4, 6, 7] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							shape[starting_index + offset] = calculated_key.get(&key).cloned();
							offset += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 9] = [
						shape[2].clone(),
						shape[1].clone(),
						shape[0].clone(),
						shape[5].clone(),
						shape[4].clone(),
						shape[3].clone(),
						shape[8].clone(),
						shape[7].clone(),
						shape[6].clone(),
					];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 2 && crafting_shaped_data.pattern.first().unwrap().len() == 2 {
				// XX
				// XX
				for starting_index in [0, 1, 3, 4] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							if offset < 2 {
								shape[starting_index + offset] = calculated_key.get(&key).cloned();
							} else {
								shape[starting_index + offset + 1] = calculated_key.get(&key).cloned();
							}
							offset += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 9] = [
						shape[2].clone(),
						shape[1].clone(),
						shape[0].clone(),
						shape[5].clone(),
						shape[4].clone(),
						shape[3].clone(),
						shape[8].clone(),
						shape[7].clone(),
						shape[6].clone(),
					];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 1 && crafting_shaped_data.pattern.first().unwrap().len() == 3 {
				// XXX
				for starting_index in [0, 3, 6] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							shape[starting_index + offset] = calculated_key.get(&key).cloned();
							offset += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 9] = [
						shape[2].clone(),
						shape[1].clone(),
						shape[0].clone(),
						shape[5].clone(),
						shape[4].clone(),
						shape[3].clone(),
						shape[8].clone(),
						shape[7].clone(),
						shape[6].clone(),
					];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 3 && crafting_shaped_data.pattern.first().unwrap().len() == 1 {
				// X
				// X
				// X
				for starting_index in [0, 1, 2] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							shape[starting_index + (offset * 3)] = calculated_key.get(&key).cloned();
							offset += 1;
						}
					}
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 2 && crafting_shaped_data.pattern.first().unwrap().len() == 3 {
				// XXX
				// XXX
				for starting_index in [0, 3] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							shape[starting_index + offset] = calculated_key.get(&key).cloned();
							offset += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 9] = [
						shape[2].clone(),
						shape[1].clone(),
						shape[0].clone(),
						shape[5].clone(),
						shape[4].clone(),
						shape[3].clone(),
						shape[8].clone(),
						shape[7].clone(),
						shape[6].clone(),
					];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else if crafting_shaped_data.pattern.len() == 3 && crafting_shaped_data.pattern.first().unwrap().len() == 2 {
				// XX
				// XX
				// XX
				for starting_index in [0, 1] {
					let mut shape: [Option<Vec<&str>>; 9] = [None, None, None, None, None, None, None, None, None];
					let mut offset = 0;
					for row in &crafting_shaped_data.pattern {
						for key in row.chars() {
							match offset {
								0 | 1 => shape[starting_index + offset] = calculated_key.get(&key).cloned(),
								2 | 3 => shape[starting_index + offset + 1] = calculated_key.get(&key).cloned(),
								_ => shape[starting_index + offset + 2] = calculated_key.get(&key).cloned(),
							}
							offset += 1;
						}
					}
					let reversed_shape: [Option<Vec<&str>>; 9] = [
						shape[2].clone(),
						shape[1].clone(),
						shape[0].clone(),
						shape[5].clone(),
						shape[4].clone(),
						shape[3].clone(),
						shape[8].clone(),
						shape[7].clone(),
						shape[6].clone(),
					];
					possible_shapes.push(reversed_shape);
					possible_shapes.push(shape);
				}
			} else {
				println!("is_recipe_a_match_3x3 encountered a shaped crafting recipe thats a weird shape:\n{crafting_shaped_data:?}");
			}

			let mut match_found = false;
			'outer: for possible_shape in possible_shapes {
				for (i, slot) in possible_shape.iter().enumerate() {
					if let Some(recipe_slot) = slot {
						if let Some(input_slot) = &slots[i] {
							if !recipe_slot.contains(&data::items::get_item_name_by_id(input_slot.id)) {
								continue 'outer;
							}
						} else {
							continue 'outer;
						}
					} else {
						if slots[i].as_ref().is_some_and(|x| x.count > 0) {
							continue 'outer;
						}
					}
				}
				match_found = true;
				break;
			}

			match_found
		}
		Recipe::CraftingShapeless(crafting_shapeless_data) => {
			let mut slots_vec = slots.to_vec();

			for ingredient in &crafting_shapeless_data.ingredients {
				let mut found_match = false;
				'ingredient_loop: for ingredient_option in ingredient {
					let tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
						data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
					} else {
						vec![ingredient_option]
					};
					for actual_ingredient_option in tag_resolved_ingredient_options {
						for slot in &mut slots_vec {
							if slot.as_ref().is_some_and(|x| data::items::get_item_name_by_id(x.id) == actual_ingredient_option) {
								*slot = None;
								found_match = true;
								break 'ingredient_loop;
							}
						}
					}
				}
				if !found_match {
					return false;
				}
			}

			slots_vec.iter().filter(|x| x.is_some()).collect::<Vec<&Option<Slot>>>().is_empty()
		}
		Recipe::CraftingTransmute(_crafting_transmute_data) => false,
		Recipe::CraftingSpecial(_crafting_special_data) => false,
		Recipe::CraftingDecoratedPot(_category) => false,
		_ => false,
	};
}

fn process_key(crafting_shaped_data: &CraftingShapedData, key: char) -> Option<Vec<&'static str>> {
	if key == ' ' {
		return None;
	}
	let possible_options = crafting_shaped_data.key.get(key.to_string().as_str()).unwrap();
	let mut all_possible_options: Vec<&str> = Vec::new();
	for ingredient_option in possible_options {
		let mut tag_resolved_ingredient_options: Vec<&str> = if ingredient_option.starts_with("#") {
			data::tags::get_item().get(ingredient_option.replace("#minecraft:", "").as_str()).unwrap().clone()
		} else {
			vec![ingredient_option]
		};
		all_possible_options.append(&mut tag_resolved_ingredient_options);
	}

	return Some(all_possible_options);
}


#[cfg(test)]
mod tests {
	use super::*;

	mod is_recipe_a_match_2x2 {
		use super::*;

		#[test]
		fn crafting_planks_from_acacia_log() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				None,
				None,
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:acacia_planks").unwrap();

			assert!(is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn crafting_planks_from_2acacia_logs_doesnt_work() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				None,
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:acacia_planks").unwrap();

			assert!(!is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn crafting_book() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:leather").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:book").unwrap();

			assert!(is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn crafting_book_from_one_paper_doesnt_work() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				None,
				None,
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:book").unwrap();

			assert!(!is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn matches_torch_recipe_left() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:coal").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:stick").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:torch").unwrap();

			assert!(is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn matches_torch_recipe_right() {
			let slots: &[Option<Slot>; 4] = &[
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:coal").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:stick").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:torch").unwrap();

			assert!(is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn doesnt_match_torch_recipe_wrong() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:coal").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:stick").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:torch").unwrap();

			assert!(!is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn matches_pressure_plate_up() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:oak_planks").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:oak_planks").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				None,
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:oak_pressure_plate").unwrap();

			assert!(is_recipe_a_match_2x2(slots, recipe));
		}

		#[test]
		fn doesnt_match_pressure_plate_wrong() {
			let slots: &[Option<Slot>; 4] = &[
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:oak_planks").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				None,
				Some(Slot {
					count: 1,
					id: data::items::get_items().get("minecraft:oak_planks").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
			];

			let all_recipes = data::recipes::get_recipes();
			let recipe = all_recipes.get("minecraft:oak_pressure_plate").unwrap();

			assert!(!is_recipe_a_match_2x2(slots, recipe));
		}
	}
}
