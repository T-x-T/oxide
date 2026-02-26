use super::*;
pub fn get_dispensers() -> HashMap<&'static str, LootTable> {
	let mut output: HashMap<&'static str, LootTable> = HashMap::new();
	output.insert("minecraft:trial_chambers/chamber", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:water_bucket"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Constant(1.0),
								add: Some(false)
							}),
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: Some(4),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:snowball"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:egg"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: Some(2),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:fire_charge"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: Some(6),
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:splash_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:slowness"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:splash_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:poison"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:splash_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:weakness"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:slowness"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:poison"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:weakness"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
					LootTablePoolEntry::Singleton(LootTablePoolEntrySingleton {
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:lingering_potion"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetPotion("minecraft:healing"),
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(2.0),), Box::new(NumberProvider::Constant(5.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:dispensers/trial_chambers/chamber"),
	});
	output.insert("minecraft:trial_chambers/corridor", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:arrow"),
					conditions: vec![],
					functions: vec![
							ItemModifier::SetCount(SetCountData {
								count: NumberProvider::Uniform(Box::new(NumberProvider::Constant(4.0),), Box::new(NumberProvider::Constant(8.0),)),
								add: Some(false)
							}),
					],
					weight: None,
					quality: None,
					}),
				],
			},
		],
		random_sequence: Some("minecraft:dispensers/trial_chambers/corridor"),
	});
	output.insert("minecraft:trial_chambers/water", LootTable {
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
					entry_type: LootTablePoolEntrySingletonType::Item("minecraft:water_bucket"),
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
				],
			},
		],
		random_sequence: Some("minecraft:dispensers/trial_chambers/water"),
	});
	return output;
}