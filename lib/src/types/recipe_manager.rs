use super::*;
use basic_types::recipe::Recipe;
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
		let now = std::time::Instant::now();
		if slots.iter().all(|x| x.is_none()) {
			return None;
		}
		let matching_recipes: Vec<(&&'static str, &Recipe)> =
			self.recipes.iter().filter(|(_, recipe)| is_recipe_a_match_2x2(slots, recipe)).collect();

		if matching_recipes.len() > 1 {
			println!("found multiple matching recipes for slots {slots:?}:\n{matching_recipes:?}");
		}

		println!("get_crafting_recipe_2x2 took {:?}", std::time::Instant::now() - now);
		if matching_recipes.is_empty() {
			return None;
		} else {
			return Some(matching_recipes.first().unwrap().1);
		}
	}
}

fn is_recipe_a_match_2x2(slots: &[Option<Slot>; 4], recipe: &Recipe) -> bool {
	return match recipe {
		Recipe::CraftingShaped(_crafting_shaped_data) => false,
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
							if slot.as_ref().is_some_and(|x| data::items::get_item_name_by_id(x.item_id) == actual_ingredient_option) {
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

#[cfg(test)]
mod tests {
	use super::*;

	mod is_recipe_a_match_2x2 {
		use super::*;

		#[test]
		fn crafting_planks_from_acacia_log() {
			let slots: &[Option<Slot>; 4] = &[
				Some(Slot {
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
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
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:acacia_log").unwrap().id,
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
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:paper").unwrap().id,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				}),
				Some(Slot {
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:leather").unwrap().id,
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
					item_count: 1,
					item_id: data::items::get_items().get("minecraft:paper").unwrap().id,
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
	}
}
