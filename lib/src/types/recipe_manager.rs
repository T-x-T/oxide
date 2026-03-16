use super::*;
use basic_types::recipe::{CraftingShapedData, Recipe};
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
