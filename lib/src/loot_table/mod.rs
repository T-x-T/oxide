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
	block_states: &HashMap<String, data::blocks::Block>,
) -> Vec<Item> {
	let block_name = data::blocks::get_block_name_from_block_state_id(block_state_id, block_states);
	let Some(loot_table) = loot_tables.get("blocks").unwrap().get(block_name.as_str()) else {
		println!("didnt find loot table for {block_name}");
		return Vec::new();
	};

	return evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name);
}

fn evaluate_loot_table(
	loot_table: &LootTable,
	block_state_id: u16,
	used_tool: &Slot,
	block_states: &HashMap<String, data::blocks::Block>,
	loot_tables: &HashMap<&'static str, HashMap<&'static str, loot_table::LootTable>>,
	block_name: String,
) -> Vec<Item> {
	let mut rng = rng();
	let mut output: Vec<Item> = Vec::new();
	for pool in &loot_table.pools {
		let valid_entries = get_valid_entries_from_pool_entries(&pool.entries, block_state_id, block_states, &Some(used_tool.clone()));
		let Some(chosen_entry) = valid_entries.choose(&mut rng) else {
			return Vec::new();
		};

		match &chosen_entry.entry_type {
			LootTablePoolEntrySingletonType::Item(item) => output.push(Item {
				id: item.to_string(),
				count: 1,
				components: Vec::new(),
			}),
			LootTablePoolEntrySingletonType::LootTableId(loot_table_id) => {
				let Some(loot_table) = loot_tables.get("blocks").unwrap().get(loot_table_id) else {
					println!("didnt find loot table for {block_name}");
					continue;
				};

				return evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name);
			}
			LootTablePoolEntrySingletonType::LootTableCustom(loot_table) => {
				return evaluate_loot_table(loot_table, block_state_id, used_tool, block_states, loot_tables, block_name);
			}
			LootTablePoolEntrySingletonType::Dynamic(_) => return Vec::new(),
			LootTablePoolEntrySingletonType::Empty => return Vec::new(),
		}
	}
	return output;
}

fn get_valid_entries_from_pool_entries(
	entries: &Vec<LootTablePoolEntry>,
	block_state_id: u16,
	block_states: &HashMap<String, data::blocks::Block>,
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
	block_states: &HashMap<String, data::blocks::Block>,
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
	block_states: &HashMap<String, data::blocks::Block>,
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
			let items: Vec<&'static str> = item_predicate
				.items
				.clone()
				.into_iter()
				.flat_map(|x| {
					if x.starts_with("#") { data::tags::get_item().get(x.replace("#", "").as_str()).unwrap_or(&Vec::new()).clone() } else { vec![x] }
				})
				.collect();

			if !items.contains(&data::items::get_item_name_by_id(used_tool.item_id)) {
				return false;
			}

			if item_predicate.count.is_none() && item_predicate.count_min.is_none() && item_predicate.count_min.is_none() {
				return true;
			}

			if let Some(count) = item_predicate.count {
				return used_tool.item_count == count;
			}

			if let (Some(count_min), Some(count_max)) = (item_predicate.count_min, item_predicate.count_max) {
				return used_tool.item_count >= count_min && used_tool.item_count <= count_max;
			}

			true
		}
		Predicate::RandomChance(number_provider) => {
			let number = match number_provider {
				NumberProvider::Constant(x) => *x,
				x => {
					println!("cant evaluate number provider of type {x:?} yet, returning 0");
					0.0
				}
			};

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
