use basic_types::item_modifier::{Function, ItemModifier};
use basic_types::loot_table::{LootTable, LootTablePoolEntry, LootTablePoolEntrySingleton, LootTablePoolEntrySingletonType};
use basic_types::predicate::Predicate;
use rand::seq::*;
use rand::{Rng, rng};

use crate::types::*;
use std::collections::HashMap;

pub fn get_block_drops(
	loot_tables: &HashMap<&'static str, HashMap<&'static str, loot_table::LootTable>>,
	block_state_id: u16,
	used_tool: &Slot,
	block_states: &HashMap<String, basic_types::blocks::Block>,
) -> Vec<Slot> {
	let block = data::blocks::get_block_from_block_state_id(block_state_id, block_states);
	let block_name = block.block_name;
	let Some(loot_table) = loot_tables.get("blocks").unwrap().get(block_name) else {
		println!("didnt find loot table for {block_name}");
		return Vec::new();
	};

	let all_items = data::items::get_items();

	if data::tags::get_block().get("needs_iron_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let iron_pickaxes = [
			all_items.get("minecraft:iron_pickaxe").unwrap().id,
			all_items.get("minecraft:golden_pickaxe").unwrap().id,
			all_items.get("minecraft:diamond_pickaxe").unwrap().id,
			all_items.get("minecraft:netherite_pickaxe").unwrap().id,
		];

		if !iron_pickaxes.contains(&used_tool.id) {
			return Vec::new();
		}
	} else if data::tags::get_block().get("needs_stone_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let stone_pickaxes = [
			all_items.get("minecraft:stone_pickaxe").unwrap().id,
			all_items.get("minecraft:copper_pickaxe").unwrap().id,
			all_items.get("minecraft:iron_pickaxe").unwrap().id,
			all_items.get("minecraft:golden_pickaxe").unwrap().id,
			all_items.get("minecraft:diamond_pickaxe").unwrap().id,
			all_items.get("minecraft:netherite_pickaxe").unwrap().id,
		];

		if !stone_pickaxes.contains(&used_tool.id) {
			return Vec::new();
		}
	} else if data::tags::get_block().get("needs_diamond_tool").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let diamond_pickaxes =
			[all_items.get("minecraft:diamond_pickaxe").unwrap().id, all_items.get("minecraft:netherite_pickaxe").unwrap().id];

		if !diamond_pickaxes.contains(&used_tool.id) {
			return Vec::new();
		}
	} else if data::tags::get_block().get("mineable/pickaxe").unwrap_or(&Vec::<&str>::new()).contains(&block.block_name) {
		let pickaxes: Vec<i32> = data::tags::get_item().get("pickaxes").unwrap().iter().map(|x| all_items.get(x).unwrap().id).collect();

		if !pickaxes.contains(&used_tool.id) {
			return Vec::new();
		}
	}

	return evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name);
}

fn evaluate_loot_table(
	loot_table: &LootTable,
	block_state_id: u16,
	used_tool: &Slot,
	block_states: &HashMap<String, basic_types::blocks::Block>,
	loot_tables: &HashMap<&'static str, HashMap<&'static str, loot_table::LootTable>>,
	block_name: &str,
) -> Vec<Slot> {
	let mut rng = rng();
	let mut output: Vec<Slot> = Vec::new();
	'pool: for pool in &loot_table.pools {
		for condition in &pool.conditions {
			if !evaluate_condition(condition, block_state_id, block_states, &Some(used_tool.clone())) {
				continue 'pool;
			}
		}

		let valid_entries = get_valid_entries_from_pool_entries(&pool.entries, block_state_id, block_states, &Some(used_tool.clone()));
		let Some(chosen_entry) = valid_entries.choose(&mut rng) else {
			continue 'pool;
		};

		match &chosen_entry.entry_type {
			LootTablePoolEntrySingletonType::Item(item) => {
				let item = Slot {
					id: data::items::get_item_id_by_name(item),
					count: 1,
					components_to_add: Vec::new(),
					components_to_remove: Vec::new(),
				};
				if chosen_entry.functions.is_empty() {
					output.push(item);
				} else {
					for function in &chosen_entry.functions {
						output.append(&mut apply_function(function, vec![item.clone()], block_state_id, block_states, &Some(used_tool.clone())));
					}
				}
			}
			LootTablePoolEntrySingletonType::LootTableId(loot_table_id) => {
				let Some(loot_table) = loot_tables.get("blocks").unwrap().get(loot_table_id) else {
					println!("didnt find loot table for {block_name}");
					continue 'pool;
				};

				output.append(&mut evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name));
			}
			LootTablePoolEntrySingletonType::LootTableCustom(loot_table) => {
				output.append(&mut evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name));
			}
			LootTablePoolEntrySingletonType::Dynamic(_) => continue 'pool,
			LootTablePoolEntrySingletonType::Empty => continue 'pool,
		}
		for function in &pool.functions {
			output = apply_function(function, output, block_state_id, block_states, &Some(used_tool.clone()));
		}
	}
	return output;
}

fn get_valid_entries_from_pool_entries(
	entries: &Vec<LootTablePoolEntry>,
	block_state_id: u16,
	block_states: &HashMap<String, basic_types::blocks::Block>,
	used_tool: &Option<Slot>,
) -> Vec<LootTablePoolEntrySingleton> {
	let mut valid_entries: Vec<LootTablePoolEntrySingleton> = Vec::new();
	for entry in entries {
		valid_entries.append(&mut get_valid_entries_from_pool_entry(entry, block_state_id, block_states, used_tool));
	}
	return valid_entries;
}

fn get_valid_entries_from_pool_entry(
	entry: &LootTablePoolEntry,
	block_state_id: u16,
	block_states: &HashMap<String, basic_types::blocks::Block>,
	used_tool: &Option<Slot>,
) -> Vec<LootTablePoolEntrySingleton> {
	let mut valid_entries: Vec<LootTablePoolEntrySingleton> = Vec::new();
	match entry {
		LootTablePoolEntry::Singleton(loot_table_pool_entry_singleton) => {
			for condition in &loot_table_pool_entry_singleton.conditions {
				if !evaluate_condition(condition, block_state_id, block_states, used_tool) {
					return Vec::new();
				}
			}
			valid_entries.push(loot_table_pool_entry_singleton.clone());
		}
		LootTablePoolEntry::Tag(loot_table_pool_entry_tag) => {
			for condition in &loot_table_pool_entry_tag.conditions {
				if !evaluate_condition(condition, block_state_id, block_states, used_tool) {
					return Vec::new();
				}
			}
			if loot_table_pool_entry_tag.expand {
				for item in data::tags::get_item().get(loot_table_pool_entry_tag.name).unwrap_or(&Vec::new()) {
					valid_entries.push(LootTablePoolEntrySingleton {
						entry_type: LootTablePoolEntrySingletonType::Item(item),
						conditions: Vec::new(),
						functions: Vec::new(),
						weight: loot_table_pool_entry_tag.weight,
						quality: loot_table_pool_entry_tag.quality,
					});
				}
			} else {
				valid_entries.push(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item(loot_table_pool_entry_tag.name),
					conditions: Vec::new(),
					functions: Vec::new(),
					weight: loot_table_pool_entry_tag.weight,
					quality: loot_table_pool_entry_tag.quality,
				});
			}
		}
		LootTablePoolEntry::Composite(loot_table_pool_entry_composite) => {
			for condition in &loot_table_pool_entry_composite.conditions {
				if !evaluate_condition(condition, block_state_id, block_states, used_tool) {
					return Vec::new();
				}
			}

			match loot_table_pool_entry_composite.entry_type {
				loot_table::LootTablePoolEntryCompositeType::Group => {
					valid_entries.append(&mut get_valid_entries_from_pool_entries(
						&loot_table_pool_entry_composite.children,
						block_state_id,
						block_states,
						used_tool,
					));
				}
				loot_table::LootTablePoolEntryCompositeType::Alternatives => {
					for entry in &loot_table_pool_entry_composite.children {
						let entries = get_valid_entries_from_pool_entry(entry, block_state_id, block_states, used_tool);
						if !entries.is_empty() {
							return entries;
						}
					}
				}
				loot_table::LootTablePoolEntryCompositeType::Sequence => {
					for entry in &loot_table_pool_entry_composite.children {
						let mut entries = get_valid_entries_from_pool_entry(entry, block_state_id, block_states, used_tool);
						if !entries.is_empty() {
							valid_entries.append(&mut entries);
						} else {
							return valid_entries;
						}
					}
				}
			}
		}
	}

	return valid_entries;
}


fn evaluate_condition(
	condition: &Predicate,
	block_state_id: u16,
	block_states: &HashMap<String, basic_types::blocks::Block>,
	used_tool: &Option<Slot>,
) -> bool {
	return match condition {
		Predicate::AllOf(predicates) => predicates.iter().all(|x| evaluate_condition(x, block_state_id, block_states, used_tool)),
		Predicate::AnyOf(predicates) => predicates.iter().any(|x| evaluate_condition(x, block_state_id, block_states, used_tool)),
		Predicate::BlockStateProperty(predicate_block_state_property) => {
			let state = data::blocks::get_raw_properties_from_block_state_id(block_states, block_state_id);
			for (property_min, property_max) in &predicate_block_state_property.properties {
				let raw_property_min = data::blocks::get_raw_properties(property_min.clone());
				let raw_property_max = data::blocks::get_raw_properties(property_max.clone());
				if raw_property_min.0 != raw_property_max.0 {
					continue;
				}
				let Some(block_state_property_value) = state.iter().find(|x| x.0 == raw_property_min.0) else {
					continue;
				};

				if raw_property_min == raw_property_max
					&& raw_property_min.1.parse::<i32>().is_err()
					&& block_state_property_value.1 != raw_property_min.1
				{
					return false;
				}

				if block_state_property_value.1.parse::<i32>().unwrap_or_default() < raw_property_min.1.parse::<i32>().unwrap_or_default()
					|| block_state_property_value.1.parse::<i32>().unwrap_or_default() > raw_property_max.1.parse::<i32>().unwrap_or_default()
				{
					return false;
				}
			}
			true
		}
		Predicate::DamageSourceProperties(_predicate_damage_source_properties) => {
			println!("dont support DamageSourceProperties predicate yet");
			false
		}
		Predicate::EnchantmentActiveCheck(_) => {
			println!("dont support EnchantmentActiveCheck predicate yet");
			false
		}
		Predicate::EntityProperties(_predicate_entity_properties) => {
			println!("dont support EntityProperties predicate yet");
			false
		}
		Predicate::EntityScores(_predicate_entity_scores) => {
			println!("dont support EntityScores predicate yet");
			false
		}
		Predicate::Inverted(predicate) => !evaluate_condition(predicate, block_state_id, block_states, used_tool),
		Predicate::KilledByPlayer => {
			println!("dont support KilledByPlayer predicate yet");
			false
		}
		Predicate::LocationCheck(_predicate_location_check) => {
			println!("dont support LocationCheck predicate yet");
			false
		}
		Predicate::MatchTool(item_predicate) => {
			let Some(used_tool) = used_tool else {
				return false;
			};
			if used_tool.id == 0 || used_tool.count == 0 {
				return false;
			}
			let items: Vec<&'static str> = item_predicate
				.items
				.clone()
				.into_iter()
				.flat_map(|x| {
					if x.starts_with("#") { data::tags::get_item().get(x.replace("#", "").as_str()).unwrap_or(&Vec::new()).clone() } else { vec![x] }
				})
				.collect();

			if !items.contains(&data::items::get_item_name_by_id(used_tool.id)) {
				return false;
			}

			if item_predicate.count.is_none() && item_predicate.count_min.is_none() && item_predicate.count_min.is_none() {
				return true;
			}

			if let Some(count) = item_predicate.count {
				return used_tool.count == count;
			}

			if let (Some(count_min), Some(count_max)) = (item_predicate.count_min, item_predicate.count_max) {
				return used_tool.count >= count_min && used_tool.count <= count_max;
			}

			true
		}
		Predicate::RandomChance(number_provider) => {
			let number = evaluate_number_provider(number_provider);

			let mut rng = rng();
			let random_number: f32 = rng.random_range(0.0..=1.0);

			random_number < number
		}
		Predicate::RandomChanceWithEnchantedBonus(_predicate_random_chance_with_enchanted_bonus) => {
			println!("dont support RandomChanceWithEnchantedBonus predicate yet");
			false
		}
		Predicate::Reference(_) => {
			println!("dont support Reference predicate yet");
			false
		}
		Predicate::SurvivesExplosion => true,
		Predicate::TableBonus(_predicate_table_bonus) => {
			println!("dont support TableBonus predicate yet");
			false
		}
		Predicate::TimeCheck(_predicate_time_check) => {
			println!("dont support TimeCheck predicate yet");
			false
		}
		Predicate::ValueCheck(_predicate_value_check) => {
			println!("dont support ValueCheck predicate yet");
			false
		}
		Predicate::WeatherCheck(_predicate_weather_check) => {
			println!("dont support WeatherCheck predicate yet");
			false
		}
	};
}

fn apply_function(
	function: &ItemModifier,
	mut items: Vec<Slot>,
	block_state_id: u16,
	block_states: &HashMap<String, basic_types::blocks::Block>,
	used_tool: &Option<Slot>,
) -> Vec<Slot> {
	let conditions = function.conditions.clone();
	for condition in conditions {
		if !evaluate_condition(&condition, block_state_id, block_states, used_tool) {
			return items;
		}
	}
	let function = function.function.clone();
	return match function {
		Function::ApplyBonus(_apply_bonus_data) => items,
		Function::CopyComponents(_copy_components_data) => items,
		Function::CopyCustomData(_copy_custom_data_data) => items,
		Function::CopyName(_) => items,
		Function::CopyState(_copy_state_data) => items,
		Function::EnchantRandomly(_enchant_randomly_data) => items,
		Function::EnchantWithLevels(_enchant_with_levels_data) => items,
		Function::EnchantedCountIncrease(_enchant_count_increase_data) => items,
		Function::ExplorationMap(_exploration_map_data) => items,
		Function::ExplosionDecay => items,
		Function::FillPlayerHead(_entity_loot_context) => items,
		Function::Filtered(_filtered_data) => items,
		Function::FurnaceSmelt => items,
		Function::LimitCount(_limit_count_data) => items,
		Function::ModifyContents(_modify_contents_data) => items,
		Function::Reference(_item_modifiers) => items,
		Function::Sequence(_item_modifiers) => items,
		Function::SetAttributes(_set_attributes_data) => items,
		Function::SetBannerPattern(_set_banner_pattern_data) => items,
		Function::SetBookCover(_set_book_cover_data) => items,
		Function::SetComponents(_data_components) => items,
		Function::SetContents(_set_contents_data) => items,
		Function::SetCount(set_count_data) => {
			let add = set_count_data.add.unwrap_or(false);
			let new_count = evaluate_number_provider(&set_count_data.count);

			for item in &mut items {
				if add {
					item.count += new_count.round() as i32;
				} else {
					item.count = new_count.round() as i32;
				}
			}
			items
		}
		Function::SetCustomData(_) => items,
		Function::SetCustomModelData(_set_custom_model_data) => items,
		Function::SetDamage(_set_damage_data) => items,
		Function::SetEnchantments(_set_enchantments_data) => items,
		Function::SetFireworks(_set_fireworks_data) => items,
		Function::SetFireworkExplosion(_set_fireworks_explosion_data) => items,
		Function::SetInstrument(_) => items,
		Function::SetItem(_) => items,
		Function::SetLootTable(_set_loot_table_data) => items,
		Function::SetLore(_set_lore_data) => items,
		Function::SetName(_set_name_data) => items,
		Function::SetOminousBottleAmplifier(_number_provider) => items,
		Function::SetPotion(_) => items,
		Function::SetRandomDyes(_number_provider) => items,
		Function::SetRandomPotion(_items) => items,
		Function::SetStewEffect(_set_stew_effect_data_effects) => items,
		Function::SetWritableBookPages(_set_writable_book_pages_data) => items,
		Function::SetWrittenBookPages(_set_written_book_pages_data) => items,
		Function::ToggleTooltips(_toggle_tooltips_data) => items,
	};
}

fn evaluate_number_provider(number_provider: &NumberProvider) -> f32 {
	return match number_provider {
		NumberProvider::Constant(x) => *x,
		NumberProvider::Uniform(min, max) => {
			let mut rng = rng();
			rng.random_range(evaluate_number_provider(min)..=evaluate_number_provider(max))
		}
		x => {
			println!("cant evaluate number provider of type {x:?} yet, returning 0");
			0.0
		}
	};
}

#[cfg(test)]
mod test {
	use crate::Slot;


	#[test]
	fn ancient_debris_drops_itself_with_diamond_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:diamond_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:ancient_debris").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);

		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:ancient_debris"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn ancient_debris_drops_nothing_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:iron_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:ancient_debris").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn diamond_ore_drops_diamond_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:iron_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:diamond_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:diamond"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn diamond_ore_drops_nothing_with_stone_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:stone_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:diamond_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn diamond_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:diamond_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn coal_ore_drops_coal_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:iron_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:coal_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:coal"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn coal_ore_drops_coal_ore_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:wooden_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:coal_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:coal"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn coal_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:coal_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn copper_ore_drops_2_to_5_raw_copper_with_iron_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:iron_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:copper_ore").unwrap().clone();

		let mut counts: Vec<i32> = Vec::new();
		for _ in 0..100 {
			let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
			assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:raw_copper"));
			counts.push(res[0].count);
		}
		counts.sort();
		counts.dedup();
		println!("{counts:?}");
		assert!(counts.contains(&2));
		assert!(counts.contains(&3));
		assert!(counts.contains(&4));
		assert!(counts.contains(&5));
	}

	#[test]
	fn deepslate_coal_ore_drops_coal_ore_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:wooden_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:deepslate_coal_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:coal"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn deepslate_coal_ore_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:deepslate_coal_ore").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}


	#[test]
	fn short_grass_drops_self_with_shears() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:shears").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:short_grass").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:short_grass"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn short_grass_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:short_grass").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		if !res.is_empty() {
			assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:wheat_seeds"));
		}
	}

	#[test]
	fn basalt_drops_basalt_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:wooden_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:basalt").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:basalt"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn basalt_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:basalt").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn copper_slab_drops_with_stone_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:stone_pickaxe").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:waxed_weathered_cut_copper_slab").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:waxed_weathered_cut_copper_slab"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn copper_slab_drops_nothing_with_wooden_pickaxe() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:waxed_weathered_cut_copper_slab").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res.len(), 0);
	}

	#[test]
	fn tall_dry_grass_drops_self_with_shears() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 1,
			id: all_items.get("minecraft:shears").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};

		let block = block_states.get("minecraft:tall_dry_grass").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		assert_eq!(res[0].id, data::items::get_item_id_by_name("minecraft:tall_dry_grass"));
		assert_eq!(res[0].count, 1);
	}

	#[test]
	fn tall_dry_grass_drops_nothing_with_no_tool() {
		let block_states = data::blocks::get_blocks();
		let all_items = data::items::get_items();
		let loot_tables = data::loot_tables::get_loot_tables();

		let used_item = Slot {
			count: 0,
			id: all_items.get("minecraft:air").unwrap().id,
			components_to_add: Vec::new(),
			components_to_remove: Vec::new(),
		};
		let block = block_states.get("minecraft:tall_dry_grass").unwrap().clone();

		let res = super::get_block_drops(&loot_tables, block.states[block.default_state].id, &used_item, &block_states);
		println!("{res:?}");
		assert_eq!(res.len(), 0);
	}
}
