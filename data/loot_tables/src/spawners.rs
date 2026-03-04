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
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(4.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:golden_carrot"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(2.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:regeneration"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strength"),
							
									conditions: vec![],
							
							},
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
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:bread"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(3),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:baked_potato"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:regeneration"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:swiftness"),
							
									conditions: vec![],
							
							},
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
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:wind_charged"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:oozing"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:weaving"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:infested"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strength"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:swiftness"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:slow_falling"),
							
									conditions: vec![],
							
							},
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
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:poison"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
							ItemModifier {
								function: Function::SetPotion("minecraft:strong_slowness"),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fire_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:wind_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier {
								function: Function::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(1.0),), Box::new(NumberProvider::Constant(3.0),)),
								add: Some(false)
							}),
							
									conditions: vec![],
							
							},
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