use super::*;
pub fn get_spawners() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:ominous/trial_chamber/consumables", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_beef"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:regeneration"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:strength"),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:spawners/ominous/trial_chamber/consumables"),
	});
	output.insert("minecraft:ominous/trial_chamber/key", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:ominous_trial_key"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:spawners/ominous/trial_chamber/key"),
	});
	output.insert("minecraft:trial_chamber/consumables", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:cooked_chicken"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:regeneration"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:swiftness"),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:spawners/trial_chamber/consumables"),
	});
	output.insert("minecraft:trial_chamber/items_to_drop_when_ominous", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:wind_charged"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:oozing"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:weaving"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:infested"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:strength"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:swiftness"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:slow_falling"),
					],
					weight: None,
					quality: None,
					}),
				],
			},
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:poison"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							ItemModifier::SetPotion("minecraft:strong_slowness"),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fire_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wind_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:spawners/trial_chamber/items_to_drop_when_ominous"),
	});
	output.insert("minecraft:trial_chamber/key", LootTable {
		loot_table_type: LootTableType::Chest,
		functions: vec![],
		pools: vec![
			LootTablePool {
				conditions: vec![],
				functions: vec![],
				rolls: NumberProvider::Constant(1.0),
				bonus_rolls: NumberProvider::Constant(0.0),
				entries: vec![
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:trial_key"),
					conditions: vec![],
					functions: vec![],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:spawners/trial_chamber/key"),
	});
	return output;
}